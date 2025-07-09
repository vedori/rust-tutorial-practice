// Most errors aren't serious enough to require the program to stop entirely
// If a function can fail for reasons that you can easily interpret and respond to
// such as trying to open a file that doesn't exist
// it makes more sense to create the file instead of terminating the process
// We would cover this situation by using Rust's Result type to handle the error

// Reminder that the Result enum is defined with two variants
// Result is defined on generic types so Rust can use them in many different scenarios
// wherer the success value and error value types may differ
/*
enum Result<T, E> {
    Ok(T),
    Err(E)
}
*/

use std::fs::File;
use std::io::ErrorKind;

// == What attempt_to_open_file does ==
// 1. Try to open specified file and get a valid file handle to it
// 2. If there is an error because there is no file try to create the specified file
//  - if that doesnt work then panic
// 3. If there is any other error panic

// Opening a file could fail so Rust uses Result
// For this example
// It could succeed and return a file handle
// Or fail because it doesn't exist or we might not have permission to access the file
fn verbose_attempt_to_open_file() {
    // The return type of File::open is a Result<T, E>
    // The T is `std::fs::File` which is a file handle
    // The E is `std::io::Error`
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Calling just this is not good enough
        // Err(error) => panic!("Problem opening the file: {error:?}"),
        // It panics no matter what and doesnt handle error scenarios that are recoverable

        // io::Error has a method `kind` to get an `io::ErrorKind` value
        Err(error) => match error.kind() {
            // creating a file could also fail so we need another match statement
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

// There is a way of writing the same program without all those nested match satements
// It involves closures which will be discussed more in chapter 13
// Closures are used with many methods defined on `Result<T, E>`
fn attempt_to_open_file() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
