use std::fmt::Display;

// It is possible to implement methods conditionally for types by
// using a trait bound with an `impl` block that uses generic type parameters

// === Working Example ===
// The type `Pair<T>` always implements the `new` function to return a
// new instance of `Pair<T>`
struct Pair<T> {
    x: T,
    y: T,
}
// Remember that `Self` is a type alias for the type of the impl block
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// The next `impl` block only implements `cmp_display` if its inner type `T`
//  - implements `PartionOrd` (which is a trait that enables comparison)
//  - implements `Display` (which is a trait that enables printing)
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// === Blanket Implementations ===
// Implementations of a trait on **any type that satifies the trait bounds**
// are called blanket implementations
// Basically implementing trait A for all types that implement trait B

// The standard library implements the `ToString` trait for all types
// that implement the `Display` trait
// ``impl<T: Display> ToString for T {}``
// Since integers implement `Display` that means they can be converted to a String
// ``let x = 3.to_sting();``
