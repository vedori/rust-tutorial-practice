#![allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // dbg macro prints and returns the value of the expression inside it
        // it prints to the stderr rather than stdout
        // IT PRINTS: 30 * scale = 60
        width: dbg!(30 * scale),
        height: 50,
    };

    // the macro takes ownership of the expression inside
    // so when dealing with data structures it makes sense
    // to pass a reference instead of the structure itslef
    dbg!(&rect1);
}
