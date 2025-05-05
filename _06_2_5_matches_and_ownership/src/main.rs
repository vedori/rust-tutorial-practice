// Be careful when an enum contains non-copyable data like a String
// A match MOVES data
// a way around this is to match on a reference if necessary

// placeholders dont move the embedded value
fn example_1() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match opt {
        // placeholders are fine for embedded values when matching
        // because the match statement doesnt move the embedded value to the _ variable
        Some(_) => println!("Some!"),
        None => println!("None!"),
    };

    // if the previous match statement had this instead
    // Some(s) => println!("Some: {}", s),
    //  vvv--------- The code below WOULD FAIL
    println!("{:?}", opt);
}

// passing a reference to a match expression avoids moving non-copyable data
fn example_2() {
    let opt: Option<String> = Some(String::from("Hello world"));

    // even though opt is &Option<String>
    // Rust will use the embedded data as &String
    // when binded in a match pattern
    // https://doc.rust-lang.org/reference/patterns.html#binding-modes
    match &opt {
        // s is of type &String
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
}

fn main() {
    example_1();
    example_2();
}
