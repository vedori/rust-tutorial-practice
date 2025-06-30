// Boxe-like structures are used by Rust's data structures
// like Vec String and HashMap to hold a **variable number** of elements

fn example() {
    let mut name = String::from("Jerry");
    // final length of name can be variable and does not need to be known at compile time
    // this is because Rust uses boxes to implement the String collection type
    name.push_str(" Can");

    // Whereas the length of an array of &str is fixed and needs to be known at compile time
    // Since it loads the data to the stack (in this case from the binary)
    let words: [&'static str; 4] = ["is", "a", "funny", "name"];
    println!("{name}{words:?}");
}
