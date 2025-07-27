// Generic types wont make your program run any slower than it would with concrete types
// Rust accomplishes this by turning generic code into specific code by filling in
// the concrete types that are used when compiled
// This process is known as monomorphization

// Consider the following
// This function defines two seperate instances of Option<T>
// One is an integer and the other is a float
fn generic_code() {
    let integer = Some(5);
    let float = Some(5.0);
}

// When Rust compiles the code in `generic_code()` the compiler reads the values
// that have been used in Option<T> and identifies that
// one is `i32` and the other is <f64>
// It then expands the generic definition of Option<T> into two definitions
// specialized to `i32` and `f64`

// Here's an idea of what the compiler does to `generic_code`
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn monomorphized_code() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
