// Immutable references are read-only
// They are also called shared references
// since there can be many immutable references to a piece of data
// Immutable references premit aliasing but disallow mutation
fn immutable_references() {
    // == L1 ==
    // v: RWO
    let mut v: Vec<i32> = vec![1, 2, 3]; // L1

    // == L2 ==
    // v: R
    // r[1,2,3]: RO
    // *r[1,2,3]: R
    let r1: &i32 = &v[2];
    let r2: &i32 = &v[2];
    let r3: &i32 = &v[2]; // L2

    // r1 does not have W perms, cannot be reassigned to a new reference
    // ERROR: r2 = &v[r3];

    // push() requires the place to have RW perms, v only has R
    // v.push(4);

    // v still has R perms
    println!("Third element is {}", v[2]);
    println!("Third element is {}", *r3); // r3 (no perms); *r3 (no perms)
    println!("Third element is {}", *r1); // r1 (no perms); *r1 (no perms)
    println!("Third element is {}", *r2); // r2 (no perms); *r2 (no perms)

    // == L3 ==
    // v (no perms)
}

// Mutable references provide mutable access to data without moving it
// They are also called unique references
// since there can only be one mutable reference to a piece of data
// at a time (the usage/lifetimes cannot overlap)
fn mutable_reference() {
    // == L1 ==
    // v: RWO
    let mut v: Vec<i32> = vec![1, 2, 3]; // L1

    // == L2 ==
    // v: R
    // num: RO
    // *num: RW
    let num: &mut i32 = &mut v[2]; // L2

    // num does not have W perms, cannot be reassigned to a new reference
    // the num place was not declared as num
    // ERROR: num = &mut v[1];

    // += operater requires the place to have RW perms
    *num += 1;

    // == L3 ==
    // v: RWO (regained perms since num is no longer being used, the lifetime ended)
    // num: (no perms)
    // *num: (no perms)
    println!("Third element is {}", *num); // L3
    println!("Vector is now {:?}", v);

    // == L4 ==
    // v: R
    // num_again: RO
    // *num_again: RW
    let num_again: &mut i32 = &mut v[2]; // L4

    // Lifetimes for mutable references to the same data cannot overlap
    // ERROR: println!("Old mutable reference points to: {}", *num);

    // += operater requires the place to have RW perms
    *num_again += 39;

    // == L6 ==
    // v: RWO (perms regained since ref's lifetime ended)
    // num_again: (no perms)
    // *num_again: (no perms)
    println!("Third element is {}", *num_again); // L6
    println!("Vector is now {:?}", v); // L7

    // == L7 ==
    // v (no perms; the lifetime of v ended)
}

fn make_mutable_ref_temporarily_read_only() {
    let mut v: Vec<i32> = vec![1, 2, 3];

    // == L1 ==
    // *num: RW
    let num: &mut i32 = &mut v[2]; // L1

    // *num is pointing to data that now being borrowed by a immutable/shared reference
    // This means *num cannot mutate until the lifetime of all immutable references
    // to that data has ended

    // == L2 ==
    // *num: R
    // *num2: R
    let num2: &i32 = &*num; // L2
    println!("{} {}", *num, *num2);

    // *num regained its W perm since lifetime of num2 has ended
    *num += 9;
    println!("{}", *num);
}
