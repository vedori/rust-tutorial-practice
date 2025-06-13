// in Rust indexing a String with [] gives you an error
// let s1 = String::from("hello");
// let bad = s1[0];

// This is because Strings contain UTF8 which may produce more than one byte per character
// An index into the string's bytes will not allways correlate to a valid unicode scalar value
fn utf8_char_lengths() {
    let hola = String::from("Hola");
    let greeting = String::from("Здравствуйте");

    // As expected the lengh of hola is 4
    // Each of its characters is one byte in utf8
    println!("Length of {hola} is {}", hola.len());

    // Even though greeting has 12 characters that is not the length
    // The answer is 24 since each Unicode value in that string takes 2 bytes of storage
    println!("Length of {greeting} is {}", greeting.len());
}

fn utf8_index_selection() {
    let hello = "Здравствуйте";
    // the first byte of 'З' is 208
    // 208 is not a valid character on its own
    // let answer = &hello[0];

    let hello = "hello";
    // Even for valid characters the first byte of 'h' would return 104 not 'h'
    // let answer = &hello[0];

    // indexing operations are expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a String,
    // because Rust would have to walk through the contents from the beginning
    // to the index to determine how many valid characters there were.
}

fn three_ways_to_interpret_strings() {
    // Rust can interpret string data as
    // 1. bytes
    // 2. scalar values
    // 3. grapheme clusters
    // Which way you chose to interpret String data depends on the program

    // “नमस्ते” as bytes
    let as_bytes = [
        224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135,
    ];
    // can call bytes() on a string to get a byte iterator
    for b in "Зд".bytes() {
        println!("{b}");
    }

    // “नमस्ते” as scalar values
    // these are char values but the some are diacritic marks
    // which aren't letters onto itself
    let scalar_values = ['न', 'म', 'स', '्', 'त', 'े'];

    // can call chars() on a string to get a char iterator
    for c in "Зд".chars() {
        println!("{c}");
    }

    // “नमस्ते” as grapheme clusters
    // maps closest to what a "letter" would be
    let grapheme_cluster = ["न", "म", "स्", "ते"];
    // third party packages can be used to parse a string as an iterator over grapheme clusters
}
