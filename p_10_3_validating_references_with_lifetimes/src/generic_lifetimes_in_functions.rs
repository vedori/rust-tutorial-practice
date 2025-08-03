// Lifetimes must be specified if it is indeterminate
// this includes functions
//
// This example will take two string slices/references and return a single string slice
// This is a scenario where we don't know the concrete values that will be passed
// And we dont know the conrete lifetimes of the arguments
// Therefore the code below fails to compile
// fn invalid_longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
//
// The takeaway is that we need to know when a reference can be valid
// and where a reference points to (region of memory)
//
// We need to add generic lifetime parameters that define the relationship
// between the references and return value so that the borrow checker
// can perform its analysis
//
// === Lifetime annotations in function signatures ===
// The annotations are put in angle brackets before the function parameters
// which is the same as generic type parameters

// Describing this lifetime annotation:
// The returned reference will be valid as long as both the parameters are valid
//
// Imagine 'a describing some region in memory that is valid
// This region doesn't need to be continuous but this generic
// lifetime can encompass the memory referenced by the two parameters
// and the returned reference
// Hence, `s1`, `s2` and the returned reference are marked with 'a
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

// All references have an implicit lifetime this example just makes them explicit by labeling them
// `y` is heap allocated but is still a region of memory included by
// the generic lifetime 'a returned by `longest()`
//
// This is important to note because the lifetimes of 'y and 'x
// are NOT the same 'x has a static lifetime whereas 'y lives
// until the end of the scope
// So generic lifetimes do not force all the references to have the same lifetime
// it ensures that the references point to valid memory by limiting where it points
//
// The reason generic lifetimes can cover references with different lifetimes
// is due to Rust's subtyping and variance
fn explicit_lifetimes<'x, 'y, 'z>() {
    let x: &'x str = "hi";
    let y: &'y str = "hello".into(); // made to be heap allocated;
    let z: &'z str = "hey";

    let l1 = longest(x, y); // 'l1
    let l2 = longest(l1, z); // 'l2
}
// Below is a graph of the lifetimes in the previous function
// Consider this to be regions of memory that hold data being referenced
// The region represents the *minimum* region that includes all referenced data
// Do not assume that it spans continuously
/*
 ___________________________________
|'z "hey"                           | <-- Stack
|-----------------------------      |
|'x "hi"                      |     |
|---------------------        |     |
| region not included | 'l1   | 'l2 |
|---------------------        |     |
|'y "hello"                   |     |
|_____________________________|     |
|___________________________________| <-- Heap
*/
