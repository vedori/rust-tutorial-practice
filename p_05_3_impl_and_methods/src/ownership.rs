// TLDR: you see an error like “cannot move out of *self”, that’s usually because
// you’re trying to call a self (owning) method on a reference like &self or &mut self
#![allow(unused)]

#[derive(Clone, Copy)]
struct Rectangle {
    width: u32,
    height: u32,
    // if name: String,
    // would only be able to derive Clone
}

// Methods must be called on structs that have the necessary permissions
// The permissions required vary for &self, &mut self, and self
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // Moves with self
    // Once this is called self is moved and loses all permissions
    fn max(self, other: Rectangle) -> Rectangle {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    fn set_to_max(&mut self, other: Rectangle) {
        // When overwriting *self, Rust will implicitly drop the data in the previous *self
        // *self = self.max(other);
        //  ^^^ *self is missing O perm
        // A way around this is to <<pass in an "owned clone">>
        // A similar solution to moving out of a collection
        // but how? (solution is #[derive(Copy, Clone)])

        /*
         * Why doesnt Rust allow it by default?
         * In this case if it were to be wrapped in unsafe it would actually run fine
         * Rectangle is strictly composed of fields that implement Copy
         * However, if it were to simply copy a pointer to a heap allocated object (shallow copy)
         * When the owning pointer goes out of scope, it will deallocate the heap data
         * despite another variable owning that pointer resulting in a double free
         *
         * The solution is to explicitly allow for a <<deep copy of structs>>
         * This is done by adding #[derive (Copy, Clone)] to the definition of Rectangle
         * An explicit solution is required to mainatain stability across API changes
         * Imagine if a String field was added (it wouldnt be able to derive Copy anymore)
         */
        *self = self.max(other);
    }
}

pub(crate) fn example() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    }; // rect: RO perms

    println!("{}", rect.area());

    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect = rect.max(other_rect);
    println!("Hello, world!");
}
