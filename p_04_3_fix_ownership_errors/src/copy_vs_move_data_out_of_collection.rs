// == Take away ==
// If a value does not own heap data then it can be copied without a move
// An exception being mutable references which prevents race conditions
// ===============
//
// The reason this is unsafe is that the reference can outlive the data
// if v is dropped before s_ref
// If both are dropped it will result in a double-free which causes undefined behavior
// This WILL happen if the string value is moved again
fn bad_copy_string_out_of_vector() {
    let v = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    // The data at *s_ref is moved instead of copied because String does not implement Copy
    // and you cannot move a value out of an immutable reference
    // The reason why String does not implement Copy is to allow for cases where
    // copying a String just copies a pointer to heap data instead of always performing
    // a deep copy
    // Basically **if a value does not own heap data then it can be copied without a move**
    todo!("let s: String = *s_ref;");
}

fn safely_copy_number_out_of_vector() {
    let v: Vec<i32> = vec![0, 1, 2];
    let n_ref: &i32 = &v[0];
    // The data at *n_ref is **copied** to n
    // All number types in Rust implement the Copy trait
    // This means instead of a move the value is copied
    let n: i32 = *n_ref;
}

// So how is it possible to safely get access to an element of a String vector?

// An immutable reference does not own heap data so it can be copied without a move
fn use_immutable_reference() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");
}

// Clone the data to avoid having a reference that can outlive data
// by persisting after a deallocation / drop
// This solution is less performant given the clone
fn use_clone() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v[0].clone();
    s.push('!');
    println!("{s}");
}

// For getting a mutable element this is the best approach rather than cloning
// Under the hood, Vec::remove() actually uses `unsafe` to accomplish this
// `unsafe` bypasses Rust's safety checks by allowing the use of “raw” pointers
// which are not checked for safety by the borrow checker.
// Essentially, it allows safe programs that would fail the borrow checker to still work
// but within an area of code that is flagged as potentially unsafe
// It uses `unsafe` because there is a copy of the value (which has a pointer to heap data)
// on the stack and vector at the same time
// BUT the vector is immediately changed to shift all elements down, effectively removing the
// value from the vector
fn use_standard_method_remove() {
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}
