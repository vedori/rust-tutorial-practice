// Your code should panic when it ends up in a bad state
// == What is a bad state ==
// When some assumption, guarantee, contract, or incariant has been broken
// Examples being invalid values, contradictory values, missing values, are passed
// AND ALSO one or more of the following:
// - is unexpected (not something like a user entering data wrong)
// - your code after this point RELIES on not being in the bad state
//   (having to check for the problem every step doesnt count)
// - there is no good way to encode the failure in the types you use

// There are cases where continuing could be insecure or harmful
// It is best to panic to alert who ever calls the code about the bug
// This helps them fix it during development
// Calling panic! also helps when calling external code that is out of your control
// and returns an invalid state with no way that you can fix it