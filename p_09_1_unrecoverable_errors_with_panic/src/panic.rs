// Sometimes bad things happen in your code, and there’s nothing you can do about it
// The program can panic

// You can cause a panic with the panic macro
fn use_panic_macro() {
    panic!("crash and burn");
}

// You can do something that causes Rust code to panic
fn do_something_bad() {
    let v = vec![1, 2, 3, 4, 5];

    v[69];
}

// When a program panics a few things happen by default
// 1. Print an failure message
// 2. Unwind and clean up the stack
// 3. Quit

// == What is Unwinding? ==
// Rust walks back up the stack and cleans up the data from each function it encounters
// Walking back takes up a lot of work and makes the binary larger
// Rust allows you to chose to immediately abort the program this ends it without cleaning up
// Cleaning up the memory will be deferred to the operating system
// To switch from unwinding to aborting add `panic = 'abort` to the appropriate `[profiles]`
// section in the Cargo.toml file
// For example to abort on panic in release mode would look like this
/*
[profile.release]
panic = 'abort'
*/
