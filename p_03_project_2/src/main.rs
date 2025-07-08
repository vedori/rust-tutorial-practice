// Generate the nth Fibonacci number. Using loops
use std::io;

fn main() {
    loop {
        println!("Enter the position for the nth fibonacci number you want.");

        let mut position = String::new();
        io::stdin()
            .read_line(&mut position)
            .expect("Failed to read from standard input!");

        let position: u16 = match position.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number.");
                continue;
            }
        };

        fib(position);

        println!("Enter 'q' to quit or any other key to continue.");

        let mut quit = String::new();
        io::stdin()
            .read_line(&mut quit)
            .expect("Failed to read from standard input!");
        let quit = quit.trim().to_lowercase();

        if quit == "q" {
            break;
        }
    }
}

fn fib(position: u16) {
    let mut sum: u16 = 1;
    let mut last_sum: u16 = 0;
    let mut temp;
    for _ in 0..position {
        temp = sum;
        sum = match sum.checked_add(last_sum) {
            Some(num) => num,
            None => {
                println!("Number too large to calculate!");
                return;
            }
        };
        last_sum = temp;
    }

    println!("The fibonacci number in position {position} is {sum}");
}
