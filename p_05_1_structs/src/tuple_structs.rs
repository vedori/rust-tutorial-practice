// Rust supports structs that look similar to tupled called tuple structs
// They are structs that dont have names associated with their fields

// Useful when you want to give the whole tuple a name
// Making the tuple a different type from other tuples
// When naming each field in a regular struct would be verbose/redundant

// A function that takes Color wouldnt take Point
struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

pub(crate) fn example() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Can still be destructured
    // type need to be specified when destructuring
    let Point(x, y, z) = origin;
    let Color(r, g, b) = black;

    println!("{x}{y}{z}");
    println!("{r}{g}{b}");
}
