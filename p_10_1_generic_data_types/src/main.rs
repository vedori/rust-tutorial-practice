#![allow(unused, clippy::useless_vec, non_camel_case_types)]
// Generics can be used to create definitions
// for many different items such as function signatures
// structs, enums

// 1
// Generics are useful for handing the duplication of concepts in Rust
mod why_use_generics;

// 2
mod generics_in_function_signatures;

// 3
mod generics_in_struct_definitions;

// 4
mod generics_in_enum_definitions;

// 5
// impl {}
mod generics_in_method_definitions;

// 6
mod performance_cost;

fn main() {
    println!("Hello, world!");
}
