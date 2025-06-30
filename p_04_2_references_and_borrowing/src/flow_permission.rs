// The borrow checker enforces that data must outlive ANY references to it
// There are two different scenarios that comes with its own way of enforcing this

// The first scenario is when references are created and dropped
// within the scope of a single function
// Rust knows exactly how long s_ref lives
// Ownership permission succeeds in this
fn single_scope() {
    // s: RO
    let s = String::from("Hello world");

    // s: R(O removed since its being borrowed by s_ref)
    let s_ref = &s;

    // drop() requires the place to have O perm
    // s does not have O perms right now
    // ERROR: drop(s);

    println!("{}", s_ref);
}

// The second scenario
// Rust needs a different mechanism when it doesn't know how long a reference lives
// This is the case when references are either input/outputs of a function
// Rust uses another kind of permission, the flow permission F
// The F persmission does not change throughout the body of a function

// A reference has the F permission if it is allowed "flow"/be used in a particular expression

// This function returns a reference to the first element in a vector
fn first_element(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];

    // The F permission is expected for references returned as an output
    // s_ref: RF
    s_ref
}
