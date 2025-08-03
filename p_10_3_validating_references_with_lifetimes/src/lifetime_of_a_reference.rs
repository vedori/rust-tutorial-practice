// Every reference has a lifetime and most times the lifetimes are implicit
// But it is required to annotate a reference's lifetime
// in scenarios when the lifetimes of references could vary
// Lifetime validation prevents dangling references
// which casuses a program to reference data other than the data it's intended to reference

// === What are lifetimes ===
// A lifetime is a specifier about how long a reference is alive for
// and a reference is alive if it's **current value**
// may be used later in the program
//
// Lifetimes are specified with labels like 'a 'b 'c when necessary
//
// Essentially a lifetime describes:
// - when a reference can be valid (region of code)
// - where a reference points to (valid region of memory)
// ==========================

// The call to `println` tries to use `r` which is a reference to stack data `x`
// `x` does not live long enough since 'r (the lifetime of `r`) is still valid
// past the scope where `x` is no longer valid (the lifetime of `r` crosses the `}`)
#[rustfmt::skip]
fn dangling_reference() {
    // r is not set to any value so the lifetime does not start here
    let r;
    {
        let x = 5;
        r = &x;                // ----+-- 'r = { &x }
    }                          //     |
    //println!("r: {}", *r);   // ----+
}

// When println is not called the code is still valid
// because the reference `r` is only valid for one line
// where x is still in scope (the lifetime of `r` does not cross the `}`)
#[rustfmt::skip]
fn dangling_reference_fixed() {
    let r;
    {
        let x = 5;
        r = &x;                // ----- 'r = { &x }
    }
}

// There is a section where the first value of `r`
// is not valid or alive before it is assigned to the reference of another variable
#[rustfmt::skip]
fn noncontinuous_lifetime() {  
    let num = 69;                
    let mut r;                 
                               
    {                          //
        let x = 42;            //
        r = &x;                // -----+-- 'r = { &x }
                               //      |
        println!("r: {}", *r); // -----+
    }                          //
                               //
    r = &num;                  // -----+-- 'r = { &num }
                               //      |
    println!("r: {}", *r);     // -----+
}

// If `r` was never reassigned it would be invalid after the scope
#[rustfmt::skip]
fn invalid_example() {
    let num = 69;                
    let mut r;                 
                               
    {                          //
        let x = 42;            //
        r = &x;                // -----+-- 'r = { &x }
                               //      |
        println!("r: {}", *r); //      |
    }                          //      |
                               //      |
    //println!("r: {}", *r);   // -----+
}

// However if 'r points to a value that is valid it can still use it
// even if it was assigned in a different scope that ended
#[rustfmt::skip]
fn valid_example() {  
    let num = 69;                 
    let mut r;                    
                                 
    {                          //
        let x = 42;            //
        r = &num;              // -----+-- 'r = { &foo }
                               //      |
        println!("r: {}", *r); //      |
    }                          //      | 
                               //      |
    println!("r: {}", *r);     // -----+
}

// Conditional assignment of a reference means that
// the lifetime for some section is indeterminate
// This function would have a compiler error if the last println ran
#[rustfmt::skip]
fn indeterminate_lifetime() {
    let num = 69;                
    let mut r;                 
                               
    {                                  //
        let x = 42;                    //
        r = &x;                        // -----+-- 'r = { &x }
                                       //      |
        println!("r: {}", *r);         //      |
    }                                  //      |
                                       //      |
    if "rand_string".is_ascii() {      //      |
        r = &num;                      //      +-- 'r = { &x, &num }
    }                                  //      |
                                       //      |
    //println!("r: {}", *r);           // -----+
}
