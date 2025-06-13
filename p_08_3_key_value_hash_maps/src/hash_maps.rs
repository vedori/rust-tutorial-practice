// Hash Maps stores a mapping of a Key to a Value using a hashing function
// Useful when you want to look up data not with an index but instead a key of any type
// Hash maps are heap allocated and like vectors, must be homogenous
// meaning all the keys must have the same type and all the values must have the same type
// Each key MUST BE UNIQUE.
// An example being a game where each team's score is a hash map
// Each key is a team's name and the value are each team's score

// We need to bring HashMap into scope from the collections portion of the standard library
// Rust does not automatically include this in the prelude.
use std::collections::HashMap;

// To create a hashmap you first make an empty generic hashmap with `new()`
// Then insert the first element which sets what types the K and V are
// Rust as of now, does not have a built-in macro to construct them
fn creating_hash_maps() -> std::collections::HashMap<std::string::String, i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}

// Use the get method to access the value with an immutable ref of the key
// The value given for the key may not be in the hash map so get returns an Option
// Specifically, the get method returns an Option of type &T that can be handled / unwrapped
fn accessing_values() {
    let scores = creating_hash_maps();
    let team_name = String::from("Blue");

    // calling copied() on the reference type turns Option<&T> to Option<T>
    // We copied the content to a new structure that we can now own
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

fn iterating_over_key_value_pairs() {
    let scores = creating_hash_maps();

    // Can use a for loop to iterate over each key-value pair in a hash map
    // Accesses each pair in an arbritary order
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
