// return_a_string() is bad code
// It attempts to return a reference to data in a function stack
// The function stack will return and the data will be automatically removed
// Yet the return value will be a refernce to the deallocated data
// The underlying string should outlive any reference to it
/*
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
*/

// == Questions to ask ==
// 1. How long should my string live?
// 2. Who should be in charge of deallocating it?

// There are four stratgies to extend the lifetime of the string in return_a_string
// Of course which strategy is most appropriate will depend on the application

// Solution #1
// You can move the ownership of the String out of the function
// by returning an owned value String instead of &String
fn return_an_owned_string() -> String {
    let s = String::from("Hello World");
    s
}

// Solution #2
// You can return a string literal which lives forever
// It has a lifetime of 'static
// This solution works if there is no intention of ever changing the string
// This will be in the binary itself which means no heap allocation
fn return_a_string_literal() -> &'static str {
    "Hello world"
}

// Solution #3
// Use garbage collection to defer borrow-checking to runtime
// This example uses a reference-counted pointer
use std::rc::Rc;
fn return_a_garbage_collected_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    // Rc only clones a pointer to s and not to the data itself
    Rc::clone(&s)
}

// Solution #4
// Have a caller provide a "slot" to put the string
// The function accepts a mutable reference to a String
// The caller is responsible for creating the string, but
// this is more memory efficient since it does not clone data
fn mutate_a_string_via_mut_ref(slot: &mut String) {
    slot.replace_range(.., "Hello world");
}
