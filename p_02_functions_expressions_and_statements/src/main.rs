fn main() {
    // y is assigned the value of a scope block which an expression that evaluates to 4
    let y = {
        let x = 3;
        x + 1 // no semicolon so it remains an expression
    };

    println!("The value of y is: {y}");

    let x = sixty_nine();

    println!("The value of x is: {x}");
    println!(
        "{}",
        f({
            let y = 1;
            y + 1
        })
    );

    // if is an expression in rust
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn f(x: i32) -> i32 {
    x + 1
}

fn sixty_nine() -> i32 {
    69
}
