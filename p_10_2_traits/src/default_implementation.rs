// You can define a default behavior for a trait
// Instead of just defining the method signature you write out a full function
// This example of a default implementaiton is not the only kind
pub trait PreviousExampleSummary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// Default implementations can call other methods in the same trait
// even if those other methods dont have a default implementaiton
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

use crate::implementing_trait_on_a_type::PostType;

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub post_type: PostType,
}

// When implementing the Summary trait only the method without a default implementation
// needs to be defined, which provides a lot of useful functionality with small implementors
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Now the default implementation of summarize can be called
// because `summarize_author` was defined for the SocialPost type
pub fn example() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, Ivermectin should only be used to treat
            horses as a dewormer",
        ),
        post_type: PostType::New,
    };
}
