#![allow(unused)]
// Rust sometimes rejects safe programs due to the way it tracks permissions

/*
 * The purpose of this function is to borrow from one field of a tuple and
 * write to a different field of a tuple.
 * !!!This in theory is safe!!!
 */
fn example_1() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    // first borrows name.0 removing its WO perms
    // name also doesnt have WO perms since it cannot be passed
    // to a function expecting RO perms on a (String, String) argument
    let first = &name.0;
    // name.1 still has W perms
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}

fn bad_example() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    // When name ref gets borrowed by function both name.0 and name.1 get W perms removed
    // Rust only sees that "some" String in the input gets borrowed
    // In actuality only the first String is
    /*
        let first = get_first(&name);
        name.1.push_str(", Esq."); // operation no longer allowed
    */
}

fn get_first(name: &(String, RefCell<String>)) -> &String {
    &name.0
}

// Defer borrow checking to runtime with the use of cells
use std::cell::RefCell;
fn example_2() {
    // Not implemented
}

fn main() {
    example_1();
    println!("Hello, world!");
}
