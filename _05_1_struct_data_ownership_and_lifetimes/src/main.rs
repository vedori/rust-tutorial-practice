#![allow(unused)]
// Uses String rather than &str
// We want each instance of this struct to own all of its data
// and for the data to be valid for as long as the entire struct is valid

// It's possible to have &str BUT lifetimes would have to be specified
// Since lifetimes ensure that data referenced is valid for as long as the struct is
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// This struct uses a named lifetime 'a
// The username and email fields must live as long as 'a
struct UserWithLifetimes<'a> {
    username: &'a str,
    email: &'a str,
    active: bool,
    sign_in_count: u64,
}

// Multiple lifetimes can be defined
// This is useful for scenarios like RAII where a struct
// manages temporary access to a resource but also stores
// some longerlived metadata

use std::fs::File;
struct FileManager<'a, 'b> {
    metadata: &'a str,     // Long-lived metadata
    file_access: &'b File, // Short-lived file access
}

// In such scenarios using multiple lifetimes ensures
// ***no dangling references*** while maintaining its intended flexibility

fn main() {
    println!("Hello, world!");
}
