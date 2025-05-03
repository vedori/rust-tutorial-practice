#![allow(unused, clippy::explicit_auto_deref)]
// In the impl block, <<ALL>> functions that aren't methods
// are called associated functions because they are associated with the impl type
// An example is String::from() which is a function defined on the String type

// Associated functions are often used for constructors
// Constructors retuen a new instance of the struct `Self``
// Cosntructors are often called new (this is just convention)

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // in this example the constructor is called square, usually would be called new
    // This would be called as Rectangle::new(number)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// Multiple impl blocks are allowed
// Multiple impl blocks are useful for generic types and traits
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut r = Rectangle {
        width: 1,
        height: 2,
    };

    // Method calls are syntactic sugar for Associated functions that
    // accept &self as the first input
    let area1 = r.area();
    let area2 = Rectangle::area(&r);

    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    // Rust will insert as many references and dereferences
    // to make the types match up <<FOR THE `self` PARAMETER>>
    let r_ptr = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });

    // r_ptr is a mutable reference BUT
    // this behavior will "downgrade" the mutable ref into a shared ref
    // this is because the underlying data is getting immutably borrowed by the function
    let area1 = r_ptr.area();
    let area2 = Rectangle::area(r_ptr);
    let area3 = Rectangle::area(&**r_ptr);

    // this means you cannot call `set_width`
    // on a value of type `&Rectangle` or `&Box<Rectangle>`

    assert_eq!(area1, area2);
    assert_eq!(area2, area3);
}
