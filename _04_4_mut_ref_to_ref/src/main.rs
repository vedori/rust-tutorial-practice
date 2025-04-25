// Immutable Reference (Shared References)
// old          R
// place        R O (W depends on mut status)
// underlying   R

// Mutable Reference (Unique references)
// old          (no READ write/execute) NOTHING
// place        R O
// underlying   R W
/* !!! MUTABLE REFERENCES CAN MUTATE THE UNDERLYING DATA !!! */

// Mutable references can be "temporarily downgraded" to read-only refs
fn main() {
    // v: R W O
    let mut v: Vec<i32> = vec![1, 2, 3];

    // L1
    // v: (none) !!!
    // num: R O
    // *num: R W
    let num = &mut v[2]; // L1

    // L2 (downgraded to 'read only ref')
    // v: (none)
    // num: R O
    // *num R (W was removed)   !!!
    // *num2 R                  !!!
    let num2 = &*num; // L2

    println!("{} {}", *num, *num2);
}

// General Info
