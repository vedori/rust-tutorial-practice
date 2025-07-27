// Generics can also be used in one or more fields of a struct

// Below defines a Point<T> struct which holds x and y coordinate values of any type
struct Point<T> {
    x: T,
    y: T,
}

fn one_generic_parameter() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // As it is defined, Point takes two fields that would resolve to the same type
    // So setting x to be a different type from y would give a compiler error
    // let wont_work = Point { x: 5, y: 4.0 };
}

// Rust convention is to name stuct names with CamelCase
// You can define a struct to accept more than one type of generic
// This will allow this Point struct to accept two different types
// The types could also be the same
struct PointWithTwoTypes<T, U> {
    x: T,
    y: U,
}

fn two_generic_parameters() {
    let integer_and_float = PointWithTwoTypes { x: 5, y: 4.0 };

    // The two arguments for the generic parameter can still be of the same type
    let both_integer = PointWithTwoTypes { x: 5, y: 10 };
    let both_float = PointWithTwoTypes { x: 1.0, y: 4.0 };
}

// It's possible to use even more generic type parameters in a definition
// but using more than a few makes the code harder to read.
// It's also an indicator that the code needs to be restructured into smaller pieces
