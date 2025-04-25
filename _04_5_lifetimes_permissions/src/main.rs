#![allow(unused)]
// A lifetime starts from a references creation to the last
// time the reference is used

// When a reference of a lower indirection "gains back" its permissions
// that is because the higher ones lifetime ended

fn main() {
    let mut x = 1;

    // L1
    // lifetime of Y begins
    // (x: R; y: RO; *y: R)
    let y = &x; // L1

    // L2
    // lifetime of Y ENDS
    // (x: RWO; y and y* none)
    let z = *y; // L2

    x += z;

    println!("{x}");
}

// !!! CODE CONTROL AFFECTS LIFETIMES
// it gets a little bit more complicated
// the function below will explore this

// *v gives out a read only sharable reference to c
// the if else branch changes when *v gets its W permissions back
// this is becaue c is being evaluated in the if branch

// REMEMBER
// *v or Vec<char> is the second indirection in the mut & chain

// BEFORE THIS FUNC CALL IT WAS LIKE THIS
// x: = vec!['c', ...]; (no permissions: value was borrowed by y as mutable reference)
// y: &mut Vec<char> = &mut x // guaranteed (R O)
// ascii_capitalize(y);

// v is (RO) as an argument
fn ascii_capitalize(v: &mut Vec<char>) {
    // if you were to
    // fails because it tries to read ownership to huh
    // all permissions were removed because *v's value
    // was borrowed as a mutable reference
    // let huh: Vec<char> = *v;

    // a reference to a vec of char still returns
    // a regular char with the Index operation
    // let regular_char = v[1];

    // c gets a sharable/immutable 'read only' ref to that regular char
    // that bubbles up to *v in the perms
    // **makes everything read only (or none) until the c lifetime is over**
    let c = &v[0];

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase(); // c lifetime over here
        // v has to wait for the end of that statement to regain W perms
        v[0] = up;
    } else {
        // c lifetime over here
        println!("Already capitalized: {:?}", v);
    }
}
