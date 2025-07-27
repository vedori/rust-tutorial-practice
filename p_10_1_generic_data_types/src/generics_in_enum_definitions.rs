// Enums can also hold generics
// In fact this is used in Option<T> an enum that
// is generic over type `T` and has two variants
enum OptionDefinition<T> {
    // The Some variant holds one value of type T
    Some(T),

    // The None varient doesnt hold any value
    None,
}

// The Result enum is an example of an enum that can hold
// multiple generic types

// The Result enum is a generic over two types `T` and `E`
// and has two variants: `Ok` which holds a value of type `T`
// and `Err` which holds a value of type `E`
enum ResultDefinition<T, E> {
    Ok(T),
    Err(E),
}
