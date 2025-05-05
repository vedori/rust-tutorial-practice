#![allow(unused)]
// Matches in Rust are exhaustive
// They must cover EVERY POSSIBILITY to be valid
// Some cases require "catch all" patterns
// The placeholder pattern `_` catches any expression that wasnt matched by the previous patters

/*
 * If you roll a 3 your player gets a fancy hat
 * If you roll a 7 your player loses a fancy hat
 * For <<all other values>> nothing happens
 */
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // `()` is an unit value (empty tuple) that represents nothing
        // no code will run here
        _ => (),
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
