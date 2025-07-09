pub fn use_panic() {
    panic!("crash and burn!!!!!!!!!!!!!!!! :D");
}

pub fn do_something_bad() {
    let v = vec![1, 2, 3, 4, 5];

    v[69];
}

// This is the output from calling use_panic()
// It shows us a panic message and it also
// shows the location in the source code where the panic occurred
// Here, it occured at the second line, fifth character of reading_a_panic_message.rs file
/*
thread 'main' panicked at src/find_panic_location.rs:2:5:
crash and burn!!!!!!!!!!!!!!!! :D
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/

// == Location of panic ==
// Sometimes the panic will come from someone else's code that our code calls
// Meaning that the filename and lin number reported by the error message
// will not be in your own files. A common example being standard library functions
// To solve this we need to configure an environment variable that expands
// the panic message to include a list of all functions that have been called to result in
// the error
// The environment variable is RUST_BACKTRACE and it can be set to 0,1, or full

// == RUST_BACKTRACE ==
// Debug symbols must be enabled to get back detailed backtraces
// Debug symbols are enabled by default when using cargo build
// or cargo run without the release flag
// Here is a backtrace of 1 when calling do_something_bad()
/*
thread 'main' panicked at src/find_panic_location.rs:8:6:
index out of bounds: the len is 5 but the index is 69
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/std/src/panicking.rs:697:5
   1: core::panicking::panic_fmt
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/6b00bc3880198600130e1cf62b8f8a93494488cc/library/core/src/panicking.rs:281:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /home/pc/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:274:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /home/pc/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:16:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /home/pc/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3376:9
   6: p_09_1_unrecoverable_errors_with_panic::find_panic_location::do_something_bad
             at ./src/find_panic_location.rs:8:6
   7: p_09_1_unrecoverable_errors_with_panic::main
             at ./src/main.rs:14:5
   8: core::ops::function::FnOnce::call_once
             at /home/pc/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
*/
