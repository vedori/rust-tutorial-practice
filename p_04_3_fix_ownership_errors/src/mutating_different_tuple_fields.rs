// Rust's borrow checker does not contain different places for a[0], a[1], ...
// It uses a single place a[_] that represents ALL indexes of `a`
// This is because Rust cannot always determine the value of an index
fn array_permissions() {
    // == L1 ==
    // a: RWO
    // a[_] RW
    let mut a = [0, 1, 2, 3]; // L1

    // == L2 ==
    // a: (no perms)
    // a[_]: (no perms)
    // x: RO
    // *x: RW
    let x = &mut a[1]; // L2

    *x += 1;

    // Imagine this
    // Rust wouldn't know the value of the index
    // let idx = a_complex_function();
    // let x = &mut a[idx];

    println!("{a:?}");
}

// If allowed to run this program would be safe but it fails Rust's borrow checker
fn read_from_one_element_and_write_to_another() {
    let mut a = [0, 1, 2, 3];

    // Reference to element that will be written to
    let e1 = &mut a[1]; // L1
    // == L1 ==
    // a:[_]: no perms (RW removed since its being borrowed)

    // Reference to element that will be read
    let e2 = &a[2];

    // This fails because `a` gave its read permission to `e1`
    todo!("*e1 += *e2;");
}

// In cases like this Rust has standard library functions that can be used
// These functions use `unsafe{}` under the hood but are guaranteed to be safe
// Try to avoid directly using unsafe unless absolutely sure about the validity of said program
// This example uses the `slice::split_at_mut` which works around the borrow checker
// to provide SEPERATE potentially mutable access to two sections of a slice
fn use_standard_library_functions() {
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);
    let e1 = &mut a_l[1];
    let e2 = &a_r[0];
    *e1 += *e2;
}

// This is how you would go about using unsafe directly
// Do not do this. Try to use library functions that implement unsafe under the hood
// if you want to write a program that in principle would fail the borrow checker
fn how_unsafe_is_used() {
    let mut a = [0, 1, 2, 3];
    let e1 = &mut a[1] as *mut i32;
    let e2 = &a[2] as *const i32;
    unsafe { *e1 += *e2; } 
}
