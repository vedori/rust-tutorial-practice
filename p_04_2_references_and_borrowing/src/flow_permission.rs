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

// This function may return a reference to data located at `strings` or  `default`
// and the lifetimes of `strings` and `default` are specified to be different.
// Rust cannot ensure that the data outlives any reference because
// if Rust looks at the function signature it is unclear whether the output &String
// is a reference to `strings` or `default`
// Rust will not allow default or strings to "flow" into the return value
fn first_element_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
    if strings.is_empty() {
        // strings: R(no F perm)
        todo!("&strings[0]");
    } else {
        // default: R(no F perm)
        todo!("default");
    }
}

fn example() {
    let strings = vec![];
    let default = String::from("default");
    // Rust does not allow the default or strings to "flow" into the return value
    let s = first_element_or(&strings, &default);
    drop(default);
    // This would read invalid data if the reference returned from first_or was &default
    println!("{}", s);
}

// This is also why you cannot return a reference to a variable on the stack
/*
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    // s_ref is a reference to a locally created variable in this function stack
    let s_ref = &s;
    // This reference will be invalidated when return_a_string returns
    // s_ref has no F perms
    todo!("s_ref");
}
*/
