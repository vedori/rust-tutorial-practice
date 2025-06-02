// There are a few ways to add to a string
fn adding_to_a_string() {
    // You can use push_str to append a string slice to a String
    // the parameter is a string slice because we dont want to take ownership of it
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);

    // Able to print s2 because push_str doesnt take ownership
    println!("s2 is {s2}");

    // The push method takes a single character as a parameter and adds it to String
    let mut s = String::from("Pus");
    s.push('h');
}

// Concatenation
// Combining two existing strings
// There are two ways to do this

// Using the + operator
fn concat_w_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 is consumed by this operation and cannot be used again
    // &s2 (or &String) is the second parameter
    let s3 = s1 + &s2;
    // this would result in a compiler error: println!("{s1}");
    // since the value of s1 is moved into the add operation
    println!("{s2}");

    // The + operator is similar to the following function definition
    // fn add(self, s: &str) -> String { ... }
    // ??? How does Rust turn &String into &str ???
    // This is done through **deref coersion**
    // Rust can coerce the &String argument into a &str or in other words
    // It turns &String into a str slice of its contents
    // &s2 into &s2[..]
}

// Using the format! macro
fn concat_w_format_macro() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Useful when using more complicated adds and a bunch of `+` is cumbersome
    // let using_add_op = s1 + "-" + &s2 + "-" + &s3;

    // returns a String
    let using_format = format!("{s1}-{s2}-{s3}");
    println!("{using_format}");

    // Also doensn't take ownership of any of its parameters
    println!("{s1}-{s2}-{s3}");
}
