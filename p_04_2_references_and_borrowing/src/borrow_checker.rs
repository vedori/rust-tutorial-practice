// === Why have borrow checking ===
// Boxes CANNOT be aliased (it moves ownership)
// However since references cannot rely on ownership to protect against undefined
// behavior there has to be a different approach
// This is solved through the borrow checker

// == How borrow checking works ===
// The idea is that variables have three kinds of permissions on their data
// Read  (R): data can be copied to another location.
// Write (W): data can be mutated.
// Own   (O): data can be moved or dropped.
// **References can temporarily remove these permissions**

// === Default behavior ===
// A variable has RO permissions on its data
// If it is annotated with `let` it has RWO permissions
// Creating a reference to data (borrowing it) causes that deata to be temporariy read-only
// until the reference is no longer in use
fn immutable_pointer_to_mutable_data() {
    // v: R W O
    let mut v: Vec<i32> = vec![1, 2, 3];

    // v:    R (W O removed since the data is being *borrowed* by num)
    // num:  R O (W not there because num wasn't declared with `mut`)
    // *num: R (*place* gained the R permission)
    // the *place* is on the left hand side of an assignment statement
    let num = &v[2];

    // After the print statement at **L1**, num is no longer in use
    // therefore v is no longer aliased/borrowed which means WO perms regained
    // v:    R W O
    // num:  (no perms)
    // *num: (no perms)
    println!("Third element is {}", *num); // <--- L1

    // (*) another println! could be added after it just means
    // the perms last for one more print statement

    // After v.push(4), v is no longer in use
    // v loses all of its permissions
    // v: (no perms)
    v.push(4);

    // == Why is num and *num specified ==
    // Accessing data through a reference is
    // not the same as manipulating the reference itself
    // The following example will show how a mutable pointer cannot mutate
    // the data if the pointee was defined as immutable
    // A mut pointer means that a different reference can be defined to the pointer variable
}

fn mutable_pointer_to_immutable_data() {
    // x:      R O
    let x = 0;

    // x:     R (O removed since its being borrowed)
    // x_ref:  R W O <--- W perms added since declared mut
    // *x_ref: R     <--- the underlying data assigned to x cannot be mutated was not declared mut
    let mut x_ref = &x;

    // A new immutable i32 data assigned to y
    let y = 2;

    // A mutable pointer can be assigned a different reference
    x_ref = &y;

    // A mutable pointer CANNOT MUTATE the underlying data if that data was not declared as mutable
    // ERROR: *x_ref += 9;
}

fn permissions_defined_on_places() {
    /*
    As was seen with *x_ref in the previous example, permissions aren't just defined on variables
    The concept expands into permissions being defined on **places**
    A place is anything you can put on the left-hand side of an assignement including
    - variables
    - derefernces of places like `*a`
    - array access of places like `a[0]`
    - fields of places like `a.0` or `a.field`
    - any combination of the above like `*((*a)[0].1)`
     */
}

// == Why do places lose permissions when they become unsused ==
// To ensure that some permissions are mutually exclusive
// In this example when the `num` place is assigned to `&v[2]` v cannot be mutated or dropped
// while num is in use (v only has R perms)
// Places hat

fn mutually_exclusive_permissions() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // v: RWO
    let num: &i32 = &v[2]; // v: (no perms), num: RO, *num: R
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num);
    // num: (no perms)
    // v: RWO
    v.push(4); // v (no perms since its no longer being used)
    // cannot use num after the push because it doesnt have permission
    // v can be used later it just means after that statement the perms on v would also go
}
