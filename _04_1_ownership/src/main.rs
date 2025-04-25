fn main() {
    let first = String::from("Ferris");
    // first no longer has ownership to the String
    let full = add_suffix(first); // full now has ownership to the String

    println!("{full}");
    // running the code below would fail
    /* println!("{full}{first}"); */
}

//name has ownership to the String
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
} // name is deallocated and the String ownership is returned
