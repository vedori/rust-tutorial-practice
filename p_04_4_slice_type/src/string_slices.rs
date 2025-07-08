// Slices let you reference a contiguous sequence of elements in a collection
// rather than the whole collection
// Since it is a reference it is non owning

// `first_word()` takes a string of words seperated by spaces
// and returns the first word it finds in that string
// If the function doesnt find a space in the string then the whole string must be one word
// so the entire string should be returned

// == Why Slices ==
// Without slices the signature would not be straightforward
// `fn first_word(s: &String) -> ?`
// There would be no way to return a part of a String as its own type
// There is a verbose alternative, it could return an index to the end of the word
// Let's explore both options

// Here is the index approach
fn first_word_pos_as_usize(s: &String) -> usize {
    let bytes = s.as_bytes();

    // Because the enumerate method returns a tuple, we can use patterns to destructure that tuple
    // Because we get a reference to the element from .iter().enumerate(), we use & in the pattern
    for (i, &item) in bytes.iter().enumerate() {
        // If & was not in the pattern we'd have to dereference `item` as `*item`
        // We can specify a byte literal in this case the space ' ' byte
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// == The Problem with Returning usize ==
// It is only meaningful in the context of the &String passed to the functions
// Since it is a seperate value from String there is no guarantee that it will
// still be valid in the future
// This is a problem if the data at &String mutates sometime in the future.

// This situation is made even worse if we write a function such as this
// fn second_word(s: &String) -> (usize, usize) {
// We would have to keep track of a starting and ending index

fn problem_with_usize() {
    let mut s = String::from("hello world");

    let word = first_word_pos_as_usize(&s);

    s.clear();

    // Cannot use word to retrieve data from `s` or an error would occur
    // The problem is having to keeep track of the usize and how its paired to its &String
}

// Use a string slice
fn string_slice_example() {
    let mut s = String::from("hello world");

    // A string slice is a reference to a portion of the String
    // using a range within the brackets
    // [starting_index..ending_index]
    // where ending_index is 1 more than the last position in the slice
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    let s2: &String = &s;

    println!("{hello}");

    s.push_str(" mutate me");

    // Since a string slice is a reference it also changes the permissions of the referenced data
    // This fails since `world` no longer has perms since `s` regained its perms by mutating
    // This prevents accessing a reference to portion of a String when its data is no longer valid
    // println!("{world}");
}

// == The internals of string slice data structure ==
// String slices are fat pointers.
// Fat pointers are references with metadata
// In the case of String slices, the metadata is the length of the string slice
// A String is essentially a pointer to a Vec<u8> that contains a length `len` and a buffer
// the buffer has a ptr that points to the string data
// A string slice bypasses the Vec<u8> structure and just points to the string data
// specifically the start of the range of data it references
// A string slice also has a len attribute which specifies the range of data it references
// This means a string_slice has more bytes associated with it since
// `&String` is a normal reference consisting of a single pointers (8 bytes on a 64-bit arch)
// A string slice consists of a pointer and a legth so 16 bytes (on a 64-bit arch)
pub fn size_of_string_ref_vs_slice() {
    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

// == ASCII only ==
// It is important to note that string slices must occur at valid UTF-8 character boundaries
// So creating a slice in the middle of a multibyte character will make the program panic
// with an error.
// More on this in the chapter 8 section “Storing UTF-8 Encoded Text with Strings”
fn using_range_syntax() {
    let s = String::from("hello");

    // you can drop the value before `..` to indicate starting from zero
    let s1 = &s[0..2];
    let s2 = &s[..2];
    assert_eq!(s1, s2);

    let len = s.len();

    // you can drop the value after `..` to indicate going to the end of the string
    let s1 = &s[3..len];
    let s2 = &s[3..];
    assert_eq!(s1, s2);

    // you can drop the value before and after `..` to indicate the whole string
    let s1 = &s[0..len];
    let s2 = &s[..];
    assert_eq!(s1, s2);
}

// When we call `first_word` we get back a single value that is tied to the underlying data
// Instad of `s: &String` we can use a string slice as a parameter
// The functions that can be done on &String can also be done on &str
// == Implicit Deref Coersions ==
// If we passed in a &String as a parameter to `first_word` Rust would automatically
// turn it to a string slice.
// This makes specifying parameters as a string slice more flexible since it can accept
// String references and String slices
// This also means that you do not require an allocation of String if you just need a string slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
