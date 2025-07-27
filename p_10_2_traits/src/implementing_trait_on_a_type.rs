/*
There are two structs that hold various amounts and kinds of text
1. A `NewsArticle` struct holds a news story filed in a particular location
2. A `SocialPost`struct has at most 280 characters along with
   metadata that indicates whether:
     - it was a new post
     - it was a repost
     - it was a reply to another post
- A media aggregator library crate named `aggregator` will display
  summaries of data that might be stored in a `NewsArticle` or `SocialPost` instance
- There needs to be a summary for each type
- ** The summary will be requested by calling a `summarize` method on an instance **
*/

// == Default Implementation ==
// You can define a default behavior for a trait
// Instead of just defining the method signature you write out a full function
// Default implementations can also call other methods in the same trait
// more on this in the next module

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// The media aggregator structs `NewsArticle` and `SocialPost`
// can now implement the Summary struct

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// NewsArticle implements the default behavior of the Summary trait
impl Summary for NewsArticle {}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub post_type: PostType,
}

#[derive(Debug)]
pub enum PostType {
    New,
    Reply,
    Repost,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// == Traits must be brough into scope to use behavior ==
// If an instance of `SocialPost` wants to run `summarize`
// then Summary would also be required to be brought into scope
// The code to bring both into scope assuming it was under the `aggregator` module would be
// `use aggregator::{SocialPost, Summary, PostType}`

// === Crate restrictions to Traits ===
// A trait can be implemented on a type only if the trait or type is local
// to the crate
// So standard library traits like `Display` can be used on a custom type
// like `SocialPost` and a trait like `Summary` that is local to this crate can
// have `Vec<T>` implement it
// This restriction is part of a property called coherence, more specifically *the orphan rule*
// which requires the parent type to be present
// This ensures that other people's code can't break your code and viceversa
// Without the rule, two crates could implement the same trait for the same type
