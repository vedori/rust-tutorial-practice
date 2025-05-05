#![allow(unused)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match is followed by an expression that could be any type
    // the result of the expression is compared to each arm
    // in the order that it is defined and the pattern is
    // the value being compared to the expression in the arm
    match coin {
        Coin::Penny => {
            println!("ABOLISH THE PENNY!");
            1
        }
        Coin::Nickel => 5, // second arm
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let x_coin = Coin::Penny;
    let y_coin = Coin::Dime;

    let money = value_in_cents(x_coin) + value_in_cents(y_coin);

    println!("I have {money} cents");
}
