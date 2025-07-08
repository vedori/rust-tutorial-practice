#![allow(unused)]
fn main() {
    let x: u8 = 150;
    let y = x.wrapping_add(x);
    let z = x.checked_add(x);

    println!("Wrapping add: {x} + {x} = {y}");
    match z {
        Some(num) => println!("{x} + {x} = {num}"),
        None => println!("Checked add went out of bounds for: {x} + {x}"),
    }

    let sat = 200u8.saturating_add(x);

    println!("What does saturating add mean?");
    println!("sat add: 200 + {x} = {sat}");

    println!("The computation is bounded to the maximum/minimum instead of over/underflowing.");

    println!("Tuple stuff");

    // Mut allows us to change the value of a variable without reassignment
    let mut tup: (i8, f64, u64) = (21, 42.6, 9_000_000_000_000);

    let (x, y, mut z) = tup;
    println!("(x: {x}, y: {y}, z: {z})");

    z = 8_323_122_543_894;
    println!("(x: {x}, y: {y}, z: {z})");

    tup.0 = 6;
    tup.1 = 6.9;
    tup.2 = 69_000_000;

    let small = tup.0;
    let ieee754 = tup.1;
    let big = tup.2;

    println!("({small}, {ieee754}, {big})");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("{}", b[2]);
}
