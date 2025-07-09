// In C, attempting to read beyond the end of a data structure is undefined behavior
// This can allow an attacker to read data at that memory location even though
// that memory does not belong to that structure
// This is called a buffer overread and can lead to security vulnerabilities
// Rust will stop execution with a panic if you try to read an element at an index
// that does not exist
fn attempt_buffer_overread() {
    let v = vec![1, 2, 3, 4, 5];

    v[69];
}
