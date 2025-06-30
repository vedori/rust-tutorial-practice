pub(crate) fn moving_data() {
    let first = String::from("Ferris");
    // first no longer has ownership to the String
    // the value is **moved** from `first` to the `full` variable
    let full = add_suffix(first); // full now has ownership to the String

    println!("{full}");
    // running the code below would fail
    // the value at `first` is freed because it was moved to `full`
    // reading it would cause undefined behavior
    /* println!("{full}{first}"); */
}

//name has ownership to the String
fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
} // name is deallocated and the String ownership is returned
