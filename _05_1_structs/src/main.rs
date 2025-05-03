#![allow(unused)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Naming defaults for struct names is CamelCase
struct SuperUser;

// can construct a new instance as the last expression in the function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // shorthand when field key and value are the same like email: email
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    // for mutability, the entire instance must be mutable
    // Rust doesn't allow only some fields to be marked as mutable
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremaill@example.com");

    let user2 = build_user(String::from("another@example.com"), user1.username.clone());

    // struct update syntax (...instance_to_copy)
    // shorthand for setting the remaining fields to the same
    // values as the instance specified
    let user3 = User {
        email: String::from("thirdemail@example.com"),
        ..user2 // the ownership of user2's other fields is moved to user3
                // (if they are heap allocated data without the Copy trait)
                // therefore something like this would fail the borrow check
                // println!("{}", user2.username);
                // The only way to avoid this is to enable deep copying by
                // impelementing the COPY TRAIT
    };

    // user2.active can still be read because bool is stack allocated
    // These types implement the Copy trait
    println!("{}", user2.active);

    // user2.sign_in_count can still be read because i32 is stack allocated
    // These types implement the Copy trait
    println!("{}", user2.sign_in_count);

    println!("{}", user3.email);
}
