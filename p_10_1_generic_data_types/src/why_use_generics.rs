// === Generics ===
// Generics are abstract stand-ins for concrete types like i32 and String (or other properties)
// They are used to express behavior without knowing what will be in place when compiled
// Examples of types that use generics are `Option<T>` and `Vec<T>`
// ================
//
// === Traits ===
// Traits are used to define behavior in a generic way
// Traits can be combined with generic types to **constrain a generic type**
// to accept only types that implement the given trait
// ==============
//
// === Lifetimes ===
// A variety of generics that give the compiler information about borrowed values
// and how references relate to each other
// ==================
//
// Generics are useful for handing the duplication of concepts in Rust
//
// This example shows duplication that can happen in code
// This is not ideal we shouldnt have to be copying instructions like this
fn find_largest_number_in_list_with_duplication() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

// However the following example will show how to remove duplication
// in a way that does not involve generic types
//
// First, there has to be a function that encapsulates the behavior
// that we want to run multiple times
// This returns a reference to the largest value in an i32 slice
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This function removes the duplication but does not use generics
// This is not ideal because the code wouldn't be able to accept
// handle other types like `u8` `i64` or even `char`
fn find_largest_number_in_list_without_generics_or_duplication() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}
