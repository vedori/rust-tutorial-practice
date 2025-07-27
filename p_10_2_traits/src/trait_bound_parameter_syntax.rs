use std::fmt::{Debug, Display};

use crate::default_implementation::{SocialPost, Summary};

pub trait Hello {
    fn speak();
}

// The `impl Trait` syntax is actually syntax sugar for a longer form
// known as a *trait bound*
// The longer form is useful for multiple parameters that implement the same trait
// and can express more complex cases

// Using the impl Trait syntax is useful when there are multiple traits as parameters
pub fn dialogue(item1: &impl Hello, item2: &impl Summary) {}

// === Multiple Parameters of One Trait Type ===
// Two parameters that implement Summary
// with the `impl Trait` syntax would look like this
// =============================================
pub fn notify1(item1: &impl Summary, item2: &impl Summary) {}

// Whereas using the **trait bound syntax** would look like the following funciton
// The trait bound syntax is good for forcing parameters to have the same type
pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

// === Specifying Multiple Trait Bounds ===
// The `+` syntax is used to specify more than one trait bound
// It restricts the parameter to a type that implements multiple specified traits

// This example makes use of display formatting and summarize on one item
pub fn notify3(item: &(impl Summary + Display)) {}

// This can also be used in the trait bound syntax
pub fn notify4<T: Summary + Display>(item: &T) {}

// === Using `where` Clauses ===
// Using too many trait bounds can make the parameter list harder to read
// Rust has an alternative syntax for specifying trait bounds
// inside a `where` clause after the function signature

// So instead of writing this:
fn multiple_trait_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    42
}

// The `where` clause can be used:
fn multiple_trait_function1<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    42
}
