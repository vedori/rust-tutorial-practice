#![allow(unused)]
// Rust normally disallows multiple mutable accesses to the same array,
// even when those accesses are disjoint.

fn bad_example() {
    let mut a = [0, 1, 2, 3];
    // Rust cannot be sure of the index at compile time if its a result of some fn
    // therefore a gives x ALL of its RWO perms to x including all indices a[_]
    let x = &mut a[1];
    // Cannot assign y the value at a[2] since it has no R perms
    // let y = &a[2];
    // *x += *y;
    println!("{a:?}");
}

// Must split into parts so Rust can enforce proper borrow-checking
// split_at_mut uses unsafe{} under the hood to achieve this
fn good_example() {
    let mut a = [0, 1, 2, 3];
    // let x = &mut a[1];
    // let y = &a[2];
    let (a_l, a_r) = a.split_at_mut(2);
    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;
    println!("{a:?}");
}

fn main() {
    good_example();
}
