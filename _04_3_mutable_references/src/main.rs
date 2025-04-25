// Immutable Reference (Shared References)
// old          R
// place        R O (W depends on mut status)
// underlying   R

// Mutable Reference (Unique references)
// old          (no READ write/execute) NOTHING
// place        R O
// underlying   R W
/* !!! MUTABLE REFERENCES CAN MUTATE THE UNDERLYING DATA !!! */

fn main() {
    // v: R W O
    let mut v = vec![1, 2, 3];

    // v: (none)
    // num: R O
    // *num: RW
    let num = &mut v[2];

    // since *num has RW this is valid
    *num += 1;

    /* L3
    v: R W O
    num: (none)
    *num: (none)
     */
    println!("THird element is {}", *num); // L3

    // L4
    // v: (none)
    println!("Vector is now {:?}", v); // L4
}
