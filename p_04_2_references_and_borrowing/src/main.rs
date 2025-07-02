#![allow(
    unused,
    clippy::borrowed_box,
    clippy::let_and_return,
    clippy::ptr_arg,
    clippy::needless_lifetimes
)]

use crate::aliasing::cannot_alias_and_mutate;

// 1
mod references;

// 2
mod dereferencing;

// 3
mod aliasing;

// 4
mod borrow_checker;

// 5
mod immutable_vs_mutable_references;

// 6
mod lifetimes;

// 7 RWOF
mod flow_permission;

fn main() {
    cannot_alias_and_mutate();
    println!("Hello, world!");
}
