// `stringify_name_with_title` is supposed to create a person's full name
// from a vector of name parts including an extra title

// When writing Rust it is important to ask for **the right level** of permissions
// Too little and a program fails, too much and it can introduce
// unexpected behavior

// There is a push() but name is an immutable ref with no W perms
fn invalid_stringify_name_with_title(name: &Vec<String>) -> String {
    // name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// Incorrect #1
// This function does run but not as intended
// **Functions should not mutate their inputs if the caller would not expect it
// If it was called `add_title_to_name` it might be expected to mutate its input
// This function mutates the name vector which is not expected
// The purpose should be to construct a String given a name vector
fn bad_solution_1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// Incorrect #2
// Taking ownership of the name Vec is bad
// It makes the name vector unusable after calling this function
// **It is very rare for Rust to take ownership of
// heap-owning data structures like Vec and String**
fn bad_solution_2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

// This solution works but is inefficient
// It is good since it takes an immutable ref of names
// The workaround is creating a new Vec
// by reading the data at name (which clone does)
// but it also recreates all the String values in name
fn okay_solution(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

// This avoids unecessary copies
// `slice::join` already copies the data in name into the string `full`
fn better_solution(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

// This accepts a String slice which is more versitile and does not strictly require
// creating a Vector object, for example an array of String can also be accepted
// as a String slice
// More on slices in chapter 4.4
fn best_solution(name: &[String]) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
