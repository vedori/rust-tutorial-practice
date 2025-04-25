#![allow(unused)]

// Unsafe because it is returning a reference to a String to another stack
// but the String is local to the function stack and will be deallocated
// ALL DATA MUST OUTLAST ITS REFERENCES
/*
fn return_a_string() -> &String {
    let s = String::from("Hello World");
    &s
}
*/

// Solution #1
// Instead of a reference return a new string value
fn return_a_string() -> String {
    String::from("Hello world")
}

// Solution #2
// return a string literal which lives forever ('static lifetime)
// it cannot be mutated BUT doesnt involve  a heap allocation
fn return_a_string_literal() -> &'static str {
    "Hello world"
}

// Solution #3
// Defer borrow checking to runtime
// Done with garbage collection such as a reference counted pointer
use std::rc::Rc;
fn return_a_gc_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

fn main() {
    println!("Hello, world!");
}
