#![allow(unused)]

// 1
mod panic;

// 2
mod why_panic;

// 3
mod find_panic_location;

fn main() {
    // find_panic_location::use_panic();
    find_panic_location::do_something_bad();
}
