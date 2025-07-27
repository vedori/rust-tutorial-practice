#![allow(unused)]

// When code panics there is no way to recover
// so it is important to know when you should panic
// and when you should handle the error with Result

// Returning Result is a good default choice when defining a function that might fail
// But there are specific scenarios that call for one or the other

// 1
mod when_prototyping_or_testing;

// 2
mod when_you_have_more_info_than_the_compiler;

// 3
mod when_continuing_is_harmful;

// 4
mod when_to_return_result;

// 5
mod how_to_reduce_error_handling;

// 6 ***
mod create_custom_types_for_validation;

fn main() {
    println!("Hello, world!");
    for i in 0..=10 {
        print!("{} ", how_to_reduce_error_handling::fib(i));
    }
    println!();
}
