// A trait defines the functionality a particular type has which other types can also share
// Traits can be used to define shared behavior in an abstract way
// **Traits bounds** can be used to specify that a generic type can be any type
// that has a certain behavior

// Different types share the same behavior if we can call the same methods on all of those types
// Traits definitions are a way to group method signatures together
// to define a set of behaviors necessary to accomplish some purpose

/*
Imagine there are various structs that hold various amounts and kinds of text for example
1. A `NewsArticle` struct holds a news story filed in a particular location
2. A `SocialPost`struct has at most 280 characters along with metadata

A media aggregator library crate named `aggregator` will display summaries of data
that might be stored in a `NewsArticle` or `SocialPost` instance
    - there needs to be a `summary` method for each type
    - ** the summary will be requested by calling the `summarize` method
    on an instance **
*/

// Notice how the trait is declared after the `trait` keyword
// The trait is `pub` so that crates depending on this crate can make use of this trait
pub trait Summary {
    // Inside the {} is a method signature that describes the behaviors
    // of the types that implement the trait
    // Notice how it ends in a semicolon
    // There can be multiple methods declared per trait
    fn summarize(&self) -> String;
}
