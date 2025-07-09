// Using match can be a bit verbose when trying to handle Result<T, E>
// Result has helper functions

// One helper function is unwrap which panics if Result returns an Err(E)
use std::fs::File;
pub fn unwrap() {
    let greeting_file = File::open("hello.txt").unwrap();
}
// The error message when calling `unwrap()`
/*
thread 'main' panicked at src/unwrap_and_expect.rs:7:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
*/

// == Use expect instead of unwrap ==
// There is another method that also panics with an message
// it is `expect` which allows for custom error messages
// This is prefered because it the message can convey intent
// and make tracking down the source of a panic easier
pub fn expect() {
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in this project");
}
// Error message from calling `expect`
/*
thread 'main' panicked at src/unwrap_and_expect.rs:22:6:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
*/
 