// There are other slice types outside of string slices

// This slice has a type of &[i32]
fn other_slices() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];
}
