// There are two ways to reference a value stored in a vector
// Use the get method()
// Index with `[]` (works the same as using an array)

// Using get() is generally better because Rust has built-in checks
// to avoid accessing invalid memory
// It returns an Optional that has to be checked with match to cover both outcomes

// However indexing makes sense in some applications like low-level embedded code
// where it is better to crash immediately than to continue in an inconsistent state

pub(crate) fn example() {
    let v = vec![1, 2, 3, 4, 5];

    // Index method
    // if there was an attempt to access the 10th element the program would panic (and crash)
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Get method
    let tenth: Option<&i32> = v.get(2);
    match tenth {
        Some(tenth) => println!("The tenth element is {third}"),
        None => println!("There is no tenth element"),
    }
}
