#![allow(unused)]

// 1
mod using_result;

// 2
mod unwrap_and_expect;

// 3
mod propagating_errors;

fn main() {
    // unwrap_and_expect::unwrap();
    unwrap_and_expect::expect();
    println!("Hello, world!");
}
