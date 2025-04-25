#!allow(unused)


/*
- Aliasing is accessing the same data through different variables
- Mutating aliased data will create problems

The vector is aliased by `num` and mutated by `v.push(4)`
This is possible because a reference is a non owning variable
*/

// PROGRAM THAT CANNOT COMPILE
/*
    // vector containing an array on the heap
    let mut v: Vec<i32> = vec![1, 2, 3];

    // reference to an element on the heap
    let num: &i32 = &v[2];

    // Resizes v and deallocates the previous array on the heap
    v.push(4);

    // Tries to access memory from the aliasing reference
    // but that memory is deallocated
    println!("Third element is {}", *num);

*/

// General Info
// Boxes CANNOT be aliased (it moves ownership)

// However since references cannot rely on ownership to protect against undefined
// behavior there has to be a different approach

// This is solved through the borrow checker (COMPILE TIME CHECK)

/*
The concept is that every variable has three kinds of permission
References can TEMPORARILY REMOVE these permissions

(R)ead
(W)rite
(O)wn

# R
- data can be copied to another location
# W
- data can be mutated
# O
- data can be moved or dropped

By default a variable has RO permissions on its data

MUT adds W

*/

// Version that can compile
// By default a variable has RO permissions on its data
fn main() {
    // v: R W O
    let mut v: Vec<i32> = vec![1, 2, 3];

    // v:    R (W O removed since the data is being *borrowed* by num)
    // num:  R O (W not there because underlying data isnt mut)
    // *num: R (*place* gained the R permission)
    // the *place* is on the left hand side of an assignment statement
    let num = &v[2];

    // *after println!(...) num is no longer in use
    // *(more like its final read before a mutation)
    // therefore v is lo longer borowed which means WO perms regained
    // v:    R W O
    // num:  (no perms)
    // *num: (no perms)
    println!("Third element is {}", *num);

    // (*) another println! could be added after it just means
    // it the perms last one more light

    // after v.push(4) v is no longer in use
    // v loses all of its permissions
    // v: (no perms)
    v.push(4);

    /* WHY IS THERE num and also *num */
    /*
    - Accessing data through a reference is
      not the same as manipulating the reference itself
     */

    // Another example

    // x:      R O
    // x_ref:  R W O
    // *x_ref: R     <------ the underlying data cannot be mutated
    let x = 0;

    //  x:      R (O removed since its being borrowed)
    //  x_ref   R W O
    // *x_ref   R
    let mut _x_ref = &x;
}
