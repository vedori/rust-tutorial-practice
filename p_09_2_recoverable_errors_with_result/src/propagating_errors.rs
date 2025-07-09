// When a function's implementation calls for something that might fail
// Instead of handling the error within the function itself
// there are scenarios where it sense to return the error to the calling code
// so that it can decide what to do
// This is called propagating the error.

use std::fs::File;

// Read has to be brought into scope for `File` to use its methods
// that implement the ones found in Read
use std::io::{self, Read};

// By propogating the error it means that the calling code has a variety of options
// For example it could
// 1. panic! and crash
// 2. use a default username
// 3. search for the username somewhere else
// etc...
fn verbose_read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("username.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        // propagates the error
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        // Returns a valid username String
        Ok(_) => Ok(username),
        // or propagates the error
        Err(e) => Err(e),
    }
}

// == The ? Operator ==
// Rust provides this operator to make error propagation easier
// and less verbose
// By applying this right after a Result expression it immediately returns the Err variant
// without having to write it out. Following code can assume that it is the T value of Ok(T)
// This means that the expected type for Err(e) should match the Err variant the expression returns
// This operator also works on Option<T> values as well

// Here's a rewrite of read_username_from_file() using the ? Operator
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("username.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// It can be made shorter by chaining ? statements
fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("username.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Reading a file into a string is fairly common
// so the standard library provides the convenient
// `fs::read_to_string` function that 
// 1. opens the file
// 2. creates a new String
// 3. reads the contents of the file
// 4. puts the contents into that String
// 5. Returns the String

// Already in scope) use std::io;
use std::fs;


// So read_username_from_file can be made even shorter
fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// The ? operator also works on Options

// It's possible there is a line but its also possible there couldnt be
// text could be an empty string slice so it would return None
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// == Limitation of the ? operator ==
// You can chain Result expressions or Option expressions
// but you cannot mix the two
// To convert from one to another you have to be explicit
// To convert from Result to an Option you have to use the `ok` method
// To convert from an Option to a Result you have to use the `ok_or` method

// == Propagating Errors from main() ==
// Usually main functions return () so 
// they cannot propagate errors but you can change the return type of main
// to be Result<(), Box<dyn Error>> and end the function with Ok(())
// Box<dyn Error> type is a trait object more on that in Chapter 18
// For now it can be read to mean "any kind of Error"
// When a main function returns a Result<(), E> 
// the executable will return a value of 0 if it returns Ok(())
// and will exit with a nonzero value if main returns an Err value
// Programs in C also return integers from executables so Rust adheres to that convention
// Specifically, the main function may return any types that implement
// the `std::process::Termination` trait which contains a function `report`
// that then returns an `ExitCode`