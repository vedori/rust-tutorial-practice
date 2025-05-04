#![allow(unused)]

// theese are the variants of the Message emum
// these could've been defined as seperate structs
// but when used for functions they can all be grouped
// as a Message (then match statement to determine which action to take)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can also define methods on enums
impl Message {
    fn call(&self) {
        todo!("Define method body here")
    }
}

fn main() {
    let m = Message::Write(String::from("hello world"));
    m.call();
    println!("Hello, world!");
}
