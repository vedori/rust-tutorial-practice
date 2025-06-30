// aliasing is accessing the same data through different variables
// pointers enable aliasing
// there are extra rules when it comes to mutation since it can be an issue

fn heap_reallocation() {
    // v allocates a heap array of a certain capacity
    // v:{ Vec: {buf: {RawVec: {inner: {RawVecInner: {ptr: { L1 }, cap:{Cap / 3}}}}}, len: 3} }
    // L1: {Unique: {pointer: {NonNull, pointer}}}
    let mut v: Vec<i32> = vec![1, 2, 3];

    // The vector is at cap of 3 so when we do a push
    // the vector has to
    // 1. **create a new allocation** with a larger capacity
    // 2. copy all the elements over
    // 3. deallocate the original heap array.
    v.push(4);
}

// When creating a reference to an ELEMENT of v (a vector's heap data)
// v is both aliased by the reference `num` and mutated by the operation v.push
// Rust does not allow this
// since the data pointed to by the immutable reference could be made invalid
// after a mutable operation of vec
// **Data should never be aliased and mutated at the same time**
pub fn cannot_alias_and_mutate() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    // v is now aliased by the reference num
    // you can now use the num pointer AND v to specify the third elent
    let num: &i32 = &v[2];
    println!("Third element aliased by pointer is {}", *num);
    println!("Third element is {}", v[2]);
    v.push(4);
    // ERROR: println!("Third element after vector mutate is {}", *num);
    // once the data is mutated you can no longer use an alias to access it
}

// Rust does not allow
fn rules_for_boxes_vs_references() {
    // Owned data can only be accessed through the owner
    // This makes it such that boxes (which are owned pointers) are invalidated
    // when the value is assigned to a new variable since the value is moved
    let a: Box<i32> = Box::new(5);
    // value at a is moved to b, a is now invalid and can never alias b
    let b = a;
    // ERROR: println!("{}", *a);

    // ==== In conclusion ====
    // Boxes CANNOT be aliased (it moves ownership)
    // However since references cannot rely on ownership to protect against undefined
    // behavior there has to be a different approach
    // This is solved through the borrow checker
}
