// #![allow(unused)]
// run rust-analyzer and look at the types it resolves to
fn main() {
    // Boxes own data on the heap (owning pointer)
    let x = Box::new(1u8);

    // references count as a pointer, it is a non owning pointer
    // each level of pointing is called an indirection
    let y = Box::new(&x);
    let z = &x;

    assert_eq!(z, *y);

    // a has 3 layers of indirection
    let a = ***y;
    println!("{a}");
}
