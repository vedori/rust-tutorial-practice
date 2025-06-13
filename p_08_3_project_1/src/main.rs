use std::vec::Vec;

use std::collections::HashMap;

use rand::Rng;
// Given a list of integers, use a vector and return the median
// (when sorted, the value in the middle position) and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.

const MIN_OCCURANCE: usize = 1;

fn main() {
    let mut rng = rand::rng();

    // Generate 20 random integers
    let size: u8 = 20;

    // The random integer will be from -10 to 10
    let mut list: Vec<i32> = Vec::new();
    for _ in 0..size {
        let integer: i32 = rng.random_range(-10..=10);
        list.push(integer);
    }

    // Sort and print the list
    list.sort();
    println!("{list:?}");

    // Find the median
    let middle_index = list.len() / 2;
    // Print the median
    if let Some(median) = list.get(middle_index) {
        println!("Median: {median}")
    }

    // Find the mode
    // Hash map <K,V> K is the number and V is the number of occurances
    let mut map = HashMap::new();
    for n in list {
        let appearances = map.entry(n).or_insert(MIN_OCCURANCE);
        *appearances += 1;
    }

    // Finds the greatest value of all K's in the hash map
    let mut greatest_occurance = MIN_OCCURANCE - 1;
    let mut mode = 0;
    for (num, occurances) in map {
        // Guaranteed at least the first element is mode
        // given the way greatest occurance was defined
        if occurances > greatest_occurance {
            greatest_occurance = occurances;
            mode = num;
        }
    }
    println!("Mode: {mode}");
}
