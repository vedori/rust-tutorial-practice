fn dereferencing() {
    // Boxes own data on the heap (owning pointer)
    let x = Box::new(1u8);

    // references count as a pointer, it is a non owning pointer
    // the derefernce operater (*) accesses the data being POINTED TO by a pointer
    // each level of pointing is called an indirection

    let y = Box::new(&x);
    let z = &x;

    assert_eq!(z, *y);

    // a has 3 layers of indirection
    let a = ***y;
    println!("{a}");
}

// Usually Rust implicitly inserts derefernces and references in certain cases
// such as calling a method with the dot operator
fn implicit_conversions() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);
}
