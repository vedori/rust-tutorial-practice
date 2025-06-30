// Move-only APIs can be inconvenient to use
// Heres an example where you want to read some strings twice

// Cannot read strings after it was moved to a function
fn cannot_read_twice() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    // data at m1 and m2 moved into the parameters of greet()
    greet(m1, m2);
    // let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}

// Had to return ownership of the strings to a new set variables
// Cumbersome and verbose ... there is a better way
fn can_read_twice_but_verbose() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet_returns_ownership(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}
fn greet_returns_ownership(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

// A reference is a non-owning pointer
// With it you can **borrow** (access) a value WITHOUT OWNNING IT
// the ampersand operator `&` is used to create a reference (or borrow)
fn can_read_twice() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet_with_references(&m1, &m2);
    let s = format!("{} {}", m1, m2);
}
fn greet_with_references(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}
