// Methods are a type of function more coupled to a struct
// Methods are defined within <<THE CONTEXT>> of a struct, enum or trait object
// Their first parameter is always `self`
// `self` represents the instance of the struct the method is being called on

// Example vvv

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Everything defined inside the `impl` block will
// be associated with the Rectangle type
impl Rectangle {
    // For methods the first parameter is always `self`
    // in an impl block, `Self` is an alias for the type being implemented
    // so in this case, `Self` is identical to `Rectangle`
    // `&self` method signature is a shorthand for `self: &Self`
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Methods could also have a signature of self
    // short for self: Self, but that would take ownership of the instance
    // Rarely, used but is useful if the call funcamentally changes the instance in
    // such a way where you want to restrict the caller from using the original instance

    // We can choose to give methods the same name as one of the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }

    // Often times methods with the same names are used as getters
    // The getters would return the type of the field they share the name with
    // Getters are useful because you can make the field private but the method public
    // This would enable a read-only access to that field as part of the type's public API
    fn height(&self) -> u32 {
        self.height
    }

    // Methods can also have more parameters
    // They can accept as input other instances of the same struct
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("The rectangle's height is {}", rect1.height());

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
