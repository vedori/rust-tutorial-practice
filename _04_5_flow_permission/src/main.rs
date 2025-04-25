#![allow(unused, clippy::ptr_arg)]
// input/output references are treated differently than references within a function body
// Rust uses the F permission to check the safety of those references

// F permission is expected when an expression uses
// an input reference foo(&strings[0])
// an output reference return &strings[0]

// F persmissoins DO NOT CHANGE throughout the body of a function

// A refernce has the F persmission if it's allowed to flow (be used)
// in a particular expression

use std::hash::Hash;

// This function creates a variable in its stack
// but tries to return a reference to that variable
// which will be used in a different context
// NO F PERMS IN s
// fn return_a_string() -> &String {
fn return_a_string() {
    //
    let s = String::from("Hello World");
    let _s_ref = &s;

    // No F perms
    todo!("return s_ref");
}

// EXAMPLE: passes
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    println!("{}", *s_ref);
    // R permission
    // F permission
    s_ref
}

// This function doesn't know if the return will come from strings
// or default
// It fails the F perm check
// fn first_or(strings: &Vec<String>, default: &String) -> &String {}

// Working example of why this function is not good
fn main() {
    let strings: Vec<String> = vec![];
    let default = String::from("default");
    // this could default or a ref from a String in &strings
    // let s = first_or(&strings, &default);
    drop(default);
    // println could allow `default` to flow into the return value
    // println!("{}", s);
}
