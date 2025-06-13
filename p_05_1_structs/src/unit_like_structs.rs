// Structs with no fields can be defined
// They are called unit-like structs since they behave similar to ()

// Can be useful when you need to implement a trait on some type
// BUT don't have any data you want to store in the type itself

/*
 * Purpose
 * Imagine that every instance of `AlwaysEqual` is always equal
 * to every instance of some Type T
 * May be used to have a KNOWN RESULT for testing purposes
 * If this **behavior was implemented** there would be no need for any data
 * to implement such behavior
 */
struct AlwaysEqual;

fn main() {
    // No need for curly brackets or parenthesis
    let _subject = AlwaysEqual;
}
