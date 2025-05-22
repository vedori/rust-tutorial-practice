// The crate root file

use crate::garden::vegetables::Asparagus;

// the garden module is declared here
// now Rust will look for the code in ./garden.rs
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
