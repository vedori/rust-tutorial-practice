// This enables debug printing in println macro (:?)
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    // By default {} in println uses the Display output format indtended for end users
    // With structs Display isnt mplemented and this will not run
    // println!("rect is {rect}");

    // `:?` tells println! to use the Debug output format
    // which can print the struct in a way useful for development
    // this will still not run unless an outer attribute is applied to the struct definition
    // the outer attribute is `#[derive(Debug)]`
    println!("rect is {rect1:?}");

    // pretty output is useful for larger structs and
    // uses {:#?} instead of {:?}
    #[derive(Debug)]
    struct SomeStruct {
        foo: i32,
        bar: i32,
        baz: i32,
        something: i32,
        a_number: u8,
        name: String,
    };

    let s = SomeStruct {
        foo: 2,
        bar: 5,
        baz: 2,
        something: 9,
        a_number: 4,
        name: String::from("A name"),
    };

    println!("big struct is {s:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
