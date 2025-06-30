// Variables live in frames
// A frame is a mapping from variables to values within a single scope
// Frames are organized into a stack of currently-called-functions
// The latest one is on top of the stack and when that function returns
// it is taken off the stack
// After a function returns, Rust deallocates the function's frame

// The stacks at the various L points are as follows
/*
# L1
| a_function: n=5 |

# L2
| plus_one: x=5   |
| a_function: n=5 |

# L3
| a_function: n=5, y=6 |
 */

fn a_function() {
    let n = 5; // L1
    let y = plus_one(n); // L3
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    // L2
    x + 1
}
