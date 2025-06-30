// When an expression reads a variable
// the variable’s value is copied from its slot in the stack frame

// In this case copying is not the most effecient way
fn variable_copied_in_stack() {
    // An array with 1 million elements all containing 0
    let a = [0; 1_000_000];
    // After this expression the value of a is copied into b
    // In this stack, both b and a will contain the value of 5
    // This is a problem since it is an array of 1 million elements
    let b = a;
}

// # Pointers
// Rust uses pointers to **transfer access** to data without copying it
// A pointer is a value that describes a location in memory.
// A common way to make a pointer is to **allocate memory in the heap**

// # Heap
// The heap is a seperate region of memory where data can live indefinately
// Heap data is not tied to a specific memory stack.
// Rust provices a construct called Box for putting data on the heap
fn point_to_heap_data() {
    let a = Box::new([0; 1_000_000]);
    // The data from a is now moved from that variable
    let mut b = a;
    // a cannot be used after this
    // println("{a}");
    b[0] = 5;
}

fn box_properties() {
    // Boxes own data on the heap (owning pointer)
    let x = Box::new(1u8);

    // references count as a pointer, it is a non owning pointer
    // each level of pointing is called an indirection
    let y = Box::new(&x);
    let z = &x;

    assert_eq!(z, *y);

    // a has 3 layers of indirection
    let a = ***y;
    println!("{a}");
}

// Rust automatically frees the heap memory when the pointer (variable that owns the box)
// is out of scope (more like when it's lifetime has ended)
fn free_heap_memory() {
    println!("Hello");
    point_to_heap_data(); // L1
    // L1 is where the heap data is free
}

// Box deallocation principle:
// If a variable owns a box, when Rust deallocates the variable’s frame,
// then Rust deallocates the box’s heap memory.
