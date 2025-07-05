#![allow(unused, clippy::let_and_return, clippy::ptr_arg)]

// 1
mod returning_ref_to_stack;

// 2
mod mutate_read_only_data;

// 3
mod reference_to_data_deallocated_by_another_alias;

fn main() {
    println!("Hello, world!");
}
