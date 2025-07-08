// String literals are string slices
fn string_literals() {
    // String literals are string slices pointing to a specific string in the binary
    // This is why they are immutable and hold a 'static lifetime since they are valid
    // for the duration of the whole program
    let s: &'static str = "Hello, world";
}
