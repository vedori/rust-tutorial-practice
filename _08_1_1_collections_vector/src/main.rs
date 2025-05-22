// Collections in rust are data structures that can store <<multiple heap allocated values>>
// Means they can grow and shrink dynamically and the ammound isn't known at compile time
// They are in the `std::collections` module which has all of its child items re-exported
// so they are specified as say std::vec instead of std::collections::vec

// Vectors store multiple values **sequentially in memory**
// Vectors can only store values of the same type
// This means vec requires a type annotation if assigned to an empty collection
// If it's a non-empty collection you can use the `vec!` macro

// They are useful when you have a list of items such as
// the lines of text in a file
// the price of items in a shopping cart

#![allow(unused)]

use std::vec::Vec;
fn main() {
    // v assigned to an empty collection, must have explicit type annotation
    let v: Vec<i32> = Vec::new();

    // the vec! macro is a convenient shorthand for assigning a non-empty vector
    let mut v = vec![1, 2, 3];

    // v needs to be declared as mut to push a value
    // `push(v)` appends `v` to the end of a collection
    v.push(4);

    println!("Hello, world!");
}
