// Permissions are returned at the end of a reference's lifetime

// A lifetime represents the range where a reference is used, specifically
// [ The creation of the reference <==========> The time it is last used]
// A lifetime of a reference is not only a continous region of code
// It can branch if there are things like if else statements

// Notice how the permissions on x are temporarily removed with the creation of the y ref
// but are returned to x once the lifetime of y has ended
fn continous_lifetime() {
    // x: RWO
    let mut x = 1;

    // x: R
    // Lifetime for y begins here
    let y = &x;
    // Lifetime for y ends at L1 or after the statement `let z = *y;`
    let z = *y; // L1
    // === L1 ===
    // x: RWO
    // y: (no perms; lifetime ended)

    x += z;
}

fn fragmented_lifetime() {
    let mut args: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let v: &mut Vec<char> = &mut args;
    let c = &v[0]; // *v: R (W temp removed since the data is immutably borrowed by c)

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_lowercase(); // *v: RW (c lifetime ended here)
        v[0] = up;
    } else {
        //L1
        // @L1: *v: RW (c lifetime ended here)
        println!("Already capitalized: {:?}", v);
    }
}
