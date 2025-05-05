#![allow(unused)]
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    /*
     * From 1999 through 2008, the United States minted quarters
     * with different designs for each of the 50 states on one side
     */
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // `state` matches the embedded value in the Coin::Quarter variant
        Coin::Quarter(state) => {
            //
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let alaskan_quarter = Coin::Quarter(UsState::Alaska);

    // will print "State quarter from Alaska!"
    // then print "Cents: 25"
    println!("Cents: {}", value_in_cents(alaskan_quarter))
}
