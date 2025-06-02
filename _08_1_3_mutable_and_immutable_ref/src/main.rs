// You cannot have a mutable and immutable reference to the same data
// in the same scope
// This is why you cannot push an item to a non mut vec
// But this restriction extends to other cases
// Even an immutable borrow from one element impacts the whole vector
#[allow(unused, clippy::useless_vec)]
fn main() {
    let mut v = vec![1, 2, 3, 4];

    // immutable reference borrow to an element in v
    let first = &v[0];

    // mutable borrow occurs in `push()` will not compile
    // v.push(6);

    // This is because vectors deal with sequential data
    // The act of pushing 6 to the vector can change what
    // memory region first points to if it has to allocate more sequential memory

    // ***AN EXCEPTION***
    // use Vec::remove can "move" a value out of a vector
}
