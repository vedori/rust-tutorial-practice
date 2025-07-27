// Generic types can be in the definitions of
// methods that are implemented on structs and enums

struct Point<T> {
    x: T,
    y: T,
}

// `T` has to be declared right after `impl` so Rust knows
// that the `T` in `Point<T>` is a generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Constraints can also be specified on generic types
// For example, this implements methods only on `Point<f32>` instances
// rather than on Point<T> instances
// Notice how `impl` doesn't need a parameterized type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ** It is not possible to implement specific and general methods of the same name this way**
// for example `distance_from_origin` can either be implemented for a concrete type such as f32
// or for all types as a generic type
// If `distance_from_origin` is implemented for both cases the Rust compiler will reject the proram

fn point_example() {
    let p = Point { x: 5, y: 10 };
    let p_f32 = Point { x: 5.0f32, y: 10.0 };

    println!("p.x = {}", p.x());
    println!("distance_from_origin: {}", p_f32.distance_from_origin());
    // This fails because `distance_from_origin` is only implemented for Point<f32> instances
    // println!("distance_from_origin: {}", p.distance_from_origin());
}

// It is important to not that the generic types defined in a struct's definition
// will not always be the same generic types that will be used
// in a method that implements the struct

// The following example demonstrates a situation where some generic parameters
// are declared with `impl` and some are declared with the method definition

// The `Coordinate` struct defines generic types `X1` and `Y1`
// but in the `mixup` method, a new Coordinate struct will be created from
// the X of the struct calling it but the Y will be from a different struct
// which is passed as an argument to `mixup`
#[derive(Debug)]
struct Coordinate<X1, Y2> {
    x: X1,
    y: Y2,
}

// The generic parameters `X1` and `Y1 ` are declared as parameters in `impl`
// because they are tied to the generic parameters in the struct definition
impl<X1, Y1> Coordinate<X1, Y1> {
    // The generic parameters `X2` and `Y2` are declared as parameters in `mixup`
    // because they are only relevant to the method
    fn mixup<X2, Y2>(self, other: Coordinate<X2, Y2>) -> Coordinate<X1, Y2> {
        Coordinate {
            x: self.x,
            y: other.y,
        }
    }
}

fn coordinate_example() {
    let c1 = Coordinate { x: 5, y: 10.4 };
    let c2 = Coordinate { x: "Hello", y: 'c' };

    println!("{c1:?}");
    println!("{c2:?}");

    let c3 = c1.mixup(c2);

    println!("{c3:?}");
}
