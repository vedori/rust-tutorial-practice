// When trying to deduplicate code it is important to consider using generics
//
// Below is an example where frequently used code is put under a function
// but generics arent used which leads to different functions that essentially do
// the same thing which is returning a reference to the largest value of a given slice
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn example_with_no_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
    println!("Hello, world!");
}

// === Using generics in function signatures ===
// When using a function that uses generic types you specify the type within `<>`
// This is the same when defining functions
// The convention is to use `T` which is short for type
//
// Rewriting the previous example to use generic types
// This restricts the T to an object that implements the PartialOrd trait
// which is necessary for a valid `>` operation
// This is good because if T was a File that operation would not be defined for it
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn example() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("The largest char is {result}");
    println!("Hello, world!");
}
