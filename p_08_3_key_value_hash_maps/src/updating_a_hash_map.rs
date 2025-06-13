// There are many different rules you could set when updating a hashmap like
// 1. You can chose to always (re)write the value
// 2. Write only when it isn't empty
// 3. Ignore the new value if the old value is not empty
// 4. Combine the new and old value, if the old value is not empty
// 5. Anything else, if the old value isnt empty

use std::collections::HashMap;

// An example where updating simply overwrites the value
fn overwrite() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // each key must be unique so inserting with the same key overwrites
    scores.insert(String::from("Blue"), 25);

    // prints {"Blue": 25}
    println!("{scores:?}");
}

// Adding a key and value only if a key isnt present
// This is a common thing to do so hash maps have an API for this called `entry`
// entry takes the key and returns an enum called Entry
// Entry represents a value that might or might not exist

// This example uses the entry API to
// check if the key for the Yellow team has a value associated with it
// If it doesn't, we insert the value 50, the same goes for the Blue team
fn update_only_if_key_is_not_present() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // {"Yellow": 50, "Blue": 10}
    println!("{scores:?}");

    // or_insert returns a mutable reference to V
    // If that key K exists, it is the old value
    // if it DOES NOT EXIST, it is the new value
    // The API makes it clean and is built with the borrow checker in mind
}

// This example counts how many times a specific word shows up in text
// The new value is computed based on the old value
// Specifically, the new value is the old value incremented by one
fn update_based_on_old_value() {
    let text = "hello world wolderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // if the key isnt found because it's the first time we've encountered the word
        // then add the K and set the value of V to be 0
        // otherwise grab the old value V
        let count: &mut i32 = map.entry(word).or_insert(0);

        // the value V is &mut so it should be dereferenced to access V
        *count += 1;
    } // mut ref lifetime ends at end of each loop so it abides by the borrowing rules

    // {"world": 2, "hello": 1, "wonderful": 1}
    println!("{map:?}");
}
