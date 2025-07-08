// The Option enum is used in cases where a value could be something or could be nothing
// This leverages the type system to ensure that both cases are met at compile time
// This functionality can prevent bugs that are extremely common in other programming languages.

// It is implemented as
/*
    enum Option<T> {
        None,
        Some(T)
    }
*/

fn example() {
    let some_number = Some(5); // type Option<i32>
    let some_char = Some('e'); // type Option<char>
    let absent_number: Option<i32> = None;

    // In languages with null you will get some kind of error if you try to
    // use a null value as a not-null value

    // It's important to note that Option<T> is not the same as T
    // v---------- so the code below does not work
    // let sum: i8 = 5 + Some(5)
    // this will give you a compiler error

    // So how do you turn Option<T> into T?
    // Use a match expression to cover both cases
    // The Rust will know that <<one case has to be T>>
    // in that case you can use it like T
    // There are other standard methods on the Optional enum
    // that can be useful for this https://doc.rust-lang.org/std/option/enum.Option.html
    match absent_number {
        Some(n) => println!("{n} - 2 = {}", n - 2),
        None => println!("There's nothing here"),
    }
    match some_number {
        Some(n) => println!("{n} + 4 = {}", n + 4),
        None => println!("There's nothing here"),
    }

    println!("Hello, world!");
}
