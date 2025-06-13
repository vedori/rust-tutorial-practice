// Convert strings to pig latin.

use random_word::Lang;
use std::vec::Vec;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    // Generates 20 random words
    let num_words: usize = 20;
    let mut words: Vec<&str> = Vec::new();
    for _ in 0..num_words {
        let word = random_word::get(Lang::En);
        words.push(word);
    }

    // Applies pig-latin rules to all words and prints them alongside the original word
    for word in words {
        if let Some(pig_latin_word) = pig_latin(word) {
            println!("{word} ==> {pig_latin_word}");
        }
    }
}

// The first consonant of each word is moved to the end of the word and ay is added,
// so first becomes irst-fay.
// Words that start with a vowel have hay added to the end instead
// (apple becomes apple-hay).
fn pig_latin(word: &str) -> Option<String> {
    let mut remaining_characters = word.chars();
    match remaining_characters.next() {
        Some(first_character) => {
            let is_vowel = VOWELS.contains(&first_character);
            if is_vowel {
                Some(format!("{word}-hay"))
            } else {
                Some(format!(
                    "{}{first_character}ay",
                    remaining_characters.as_str()
                ))
            }
        }
        None => None,
    }
}
