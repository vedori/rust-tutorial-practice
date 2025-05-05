#![allow(unused)]
// `if let` combines `if` and `let` to handle values that <<match one pattern while ignoring the rest>>

// only wants to execute code if the value is the Some variant
fn match_example() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is configured to be {max}"),
        None => (), // if there is a need to add this change to `if let`
    }
}

fn if_let_example() {
    let config_max = Some(3u8);

    // if let PATTERN = EXPRESSION
    // basically a match statement for ONE pattern
    if let Some(max) = config_max {
        // `max` binds to the embedded value in `config_max`
        println!("The max is configured to be {max}")
    }
}

#[derive(Debug)]
enum USStates {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USStates),
}

fn coin_match_example() {
    let mut count = 0;
    let coin = Coin::Quarter(USStates::Alabama);

    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}"),
        _ => count += 1,
    }
    println!("Count is {count}");
}

fn coin_if_let_else_example() {
    let mut count = 0;
    let coin = Coin::Dime;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}")
    } else {
        count += 1;
    }
    println!("Count is {count}");
}

#[derive(Debug)]
enum Location {
    Point(i32),

    Range(i32, i32),
}

fn print_range_max(loc: &Location) {
    // print the second field of Range, if loc is a Range
    if let Location::Range(a, b) = loc {
        println!("{b}");
    }
}

fn main() {
    match_example();
    if_let_example();
    coin_match_example();
    coin_if_let_else_example();

    let loc = Location::Range(7, 9843);
    print_range_max(&loc);
}
