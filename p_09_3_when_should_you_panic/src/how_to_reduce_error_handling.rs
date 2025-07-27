// Aside from the ? operator, a good way to reduce error handling
// is to use concrete types in your parameters
// Instead of having a parameter accept an Optional<T> or Result<T> you can
// make it accept T so that it's guaranteed that T is the type you need
fn do_not_pass_option_parameter() {
    let user_num :Option<i32>  = num_from_user();

    if let Some(n) = user_num {
        // you can be sure that n is an i32
        print_num_plus_one(n);
    }
}

// No need to error handle inside function
fn print_num_plus_one(n: i32) {
    println!("{}", n + 1);
}

fn num_from_user() -> Option<i32> {
    Some(42)
}

// Another example is to use unsigned integers such as u32
// When you are sure an integer should never be negative
// This elimanates having to check for negative
pub fn fib(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}