// One way to move data is to clone it
// It is a way for keeping access to the original data while changing a copy of it
// It is a deep copy of the data into a new heap allocation
// This means it can be resource intensive

fn cloning_data() {
    let first = String::from("Ferris");
    // It is not a shallow copy, it's not like the pointer was moved (unlike the previous module)
    // There are now two instances of "Ferris" on the heap
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
