#![allow(unused, clippy::ptr_arg)]

/*
The goal is to return the first word of any string reference
The word is all the characters before the first space ' '
If there are no spaces then the whole word is returned
 */

// If theres no way to return a reference to "subset" of a string
// then it would make sense to just return the index
// however this becomes tedious having to call this after each change
// or implementing another function that is second_words that would have two more usize
fn first_word_index(s: &String) -> usize {
    match s.find(' ') {
        Some(i) => i,
        None => s.len(),
    }
}

// There is a better solution which is using slices

// A slice is created by specifying [starting_index..ending_index]
// **ending_index is one more than the last position (exclusive)
// if starting_index is omitted it starts from 0
// if ending_index is omitted it stops at len

// A slice is a special kind of reference
// they are "fat" pointers (pointers with metadata)
// the metadata is the length of the slice

// change the input type from &String to &str allows
// us to use the same function on Strings and string literals
// **this is possible through implicit deref coercions
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let mut s = String::from("Hello Rustaceans!");

    let word = first_word(&s);

    let second_word = first_word("world wide web");

    println!("{word}, {second_word}!");
}
