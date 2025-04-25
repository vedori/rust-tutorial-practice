#![allow(clippy::ptr_arg)]
/*
 * The purpose of the function is to give back a name
 * with some prefix attached to it.
 * However, this fails to compile
```
fn stringify_name_with_title(name: &Vec<String>) -> String {
    // R O perms no W perms
    name.push(String::from("Esq."));
    let full = name.join("");
    full
}
```
 * That function fails because any other immutable name ref outside this
 * function scope would point to deallocated data
*/

// Bad Fix #1: Change parameter to a mutable reference
/*
fn stringify_name_with_title(name: &mut Vec<String>) -> String {
*/
// Problem
// This changes the value of the argument
// this is unexpected given the purpose of this function

// Bad Fix #2: Change parameter to the Vec data structure (heap allocated DS)
/*
fn stringify_name_with_title(mut name: Vec<String>) -> String {
*/
// Problem
// This takes ownership of the argument making it unusable outside the function scope
// and is a bad way to design an API in Rust

// Bad Fix #3: Clone the input name and work on the cloned version (not as bad)
/*
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
*/
// Problem
// The clone copies every string in the input
// The suffix should be added later to avoid unecessary copies

// Solution #1: use slice::join
// slice::join reads in (copies) data in `name`` into the String `full`
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}
