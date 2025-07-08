// Convert temperatures between Fahrenheit and Celsius.
use std::{f64, io};

fn main() {
    println!("Farenheit <--> Celsius converter");

    loop {
        let mut choice = String::new();

        println!("What unit do you want to convert to?");
        println!("Enter 'f' for Farenheit.");
        println!("Enter 'c' for Celsius.");
        println!("Enter 'q' to quit.");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read standard input!");

        choice = choice.trim().to_lowercase();

        if choice == "q" {
            break;
        }

        if choice != "f" && choice != "c" {
            continue;
        }

        loop {
            let mut temp = String::new();
            let mut quit_current_conversion = String::new();

            println!(
                "Enter temperature in {}",
                if choice == "f" { "C" } else { "F" }
            );

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read standard input!");

            let temp: f64 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

            let temp: f64 = if choice == "f" {
                celsius_to_farenheit(temp)
            } else {
                farenheit_to_celsius(temp)
            };

            println!("{temp}{}", choice.to_uppercase());

            println!("Enter 'q' to quit or any other key to continue.");
            io::stdin()
                .read_line(&mut quit_current_conversion)
                .expect("Failed to read standard input!");

            if quit_current_conversion.trim().to_lowercase() == "q" {
                break;
            };
        }
    }
}

fn farenheit_to_celsius(temp: f64) -> f64 {
    let temp = temp - 32.0;
    temp / 1.8
}

fn celsius_to_farenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}
