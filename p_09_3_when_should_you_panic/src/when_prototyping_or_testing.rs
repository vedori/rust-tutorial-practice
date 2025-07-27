// WHen prototyping or making tests it's more approriate to write code
// that panics instead of returning a Result

// == Prototyping ==
// When prototyping you want to have a clear vision of what you want the code to do
// Adding error handling can muddy the waters
// If there is an unwrap or expect it is a visual placeholder for error handling

// == Testing ==
// If a method call fails in a test the whole test should fail
// Panic is how a test is marked as a failure