use rand::random_range;
use std::cmp::Ordering;
use std::io;

fn main() {
    let random_number: u32 = random_range(1..=100);
    let mut tries: u32 = 0;

    println!("Try to guess the right number between 1 and 100!");

    loop {
        tries += 1;

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from standard input");

        // old bad solution
        // // had input as the string buffer and guess as the parsed int
        // let guess: u8 = input.trim().parse().unwrap_or(0);
        // if guess == 0 {
        //     println!("Enter a number!");
        //     continue;
        // }

        // <Result> objects can be have their Ok or Err values "deconstructed"
        // and evaluated with a match statement
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You won in {tries} tries!");
                break;
            }
        };
    }
}
