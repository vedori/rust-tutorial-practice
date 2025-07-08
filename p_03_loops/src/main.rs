fn main() {
    // Break expressions can return a value
    let mut count = 0;

    // Loop labels can specify which loop `break` and `continue` works on
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            // With nested loops, `break` and `continue` apply to the innermost loop
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loops
    println!("While loop implementation");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!");

    // For Loops
    // Often better since you cannot go out of bounds for certain operations
    println!("For loop implementation");
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!");
}
