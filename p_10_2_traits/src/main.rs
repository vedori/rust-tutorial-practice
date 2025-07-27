#![allow(unused)]

// 1
mod what_is_a_trait;

// 2
mod implementing_trait_on_a_type;

// 3
mod default_implementation;

// 4
mod traits_as_parameters;

// 5
mod trait_bound_parameter_syntax;

// 6
mod traits_as_return_types;

// 7
mod conditionally_implement_methods_using_trait_bounds;

fn main() {
    use implementing_trait_on_a_type::{PostType, SocialPost, Summary};

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        post_type: PostType::New,
    };

    println!("1 new post: {}", post.summarize())
}
