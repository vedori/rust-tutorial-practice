#![allow(unused, clippy::let_and_return, clippy::ptr_arg)]

// 1
mod returning_ref_to_stack;

// 2
mod mutate_read_only_data;

// 3
mod reference_to_data_deallocated_by_another_alias;

// 4
mod copy_vs_move_data_out_of_collection;

// 5
mod mutating_different_tuple_fields;

fn main() {
    println!("Hello, world!");
}
