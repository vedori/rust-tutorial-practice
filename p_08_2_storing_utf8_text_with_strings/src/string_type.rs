// Rust only has one string type which is the string slice `str` which is encoded as
// a type of byte sequence.
// Usually in it's borrowed form `&str` and is UTF-8 encoded
// &str is a promise that a byte sequence can always be interpreted as UTF-8
// this differentiates it from &[u8] which can be any byte sequence.
fn example() {
    // String literals are string slices of the characters stored in the program's binary
    let data = "initial contents";

    // The String type is not coded into the core of the language
    // but is a growable, mutable, owned and is UTF-8 encoded.
    // String is a collection so Vec<T> share many of the same operations
    // Technically String is a wapper around a vector of bytes with some extra features
    let s = "literal to owned String".to_string();
    let s = data.to_string();

    // Non ASCII characters can be put in strings since its UFT-8
    let hello_from_japan = String::from("こんにちは");
}
