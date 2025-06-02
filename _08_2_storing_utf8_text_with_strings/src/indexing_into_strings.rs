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
