use crate::default_implementation::Summary;

// Using traits as parameters allows you to define functions that can
// accept many different types as long as each of those types implements the trait
// The `impl` keyword along with the trait name is specified
// for the `item` parameter instead of a concrete type
// This is known as the `impl Trait` syntax

// The following function can call `summarize()` on any argument to `item`
// that implements the Summary trait
pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}
