use crate::default_implementation::{SocialPost, Summary};
use crate::implementing_trait_on_a_type::{NewsArticle, PostType};

// The `impl Trait` syntax can also be used in the return value
// This signals that the return value is some type that implements a trait
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, Ivermectin should only be used to treat
            horses as a dewormer",
        ),
        post_type: PostType::New,
    }
}

// === Why is returning a trait instead of a concrete type useful? ===
// In the context of closures and iterators (more on this in Chapter 13)
// they create types that only the compiler knows or types that are very long to specify
// So `impl Trait` syntax allows you to specify that a function returns some
// type that implements the `Iterator` trait without needing to write out a very long type

// However the `impl Trait` syntax can only be used if you're returning a single type
// So a function that returns either a `NewsArticle` or a `SocialPost` with
// the return type specified as `impl Summary` wouldn't work
/*
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            post_type: PostType::New,
        }
    }
}
*/

// Returning either a NewsArticle or a SocialPost isn’t allowed
// due to restrictions around how the impl Trait syntax is implemented in the compiler.
// It is technically possible to write a function with this behavior
// More on that in Chapter 18
