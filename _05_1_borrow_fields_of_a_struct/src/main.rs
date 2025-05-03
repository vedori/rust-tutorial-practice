fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let mut p = Point { x: 0, y: 0 };

    // field x is borrowed
    // p and p.x temporarily lose their permissions
    // p.y does not lose its permission
    let x = &mut p.x;

    // Therefore this would be invalid here
    // println!("{}", p.x);
    *x += 1;

    // but over here is fine to access p.x since
    // the lifetime of x is over

    // You can take a &mut to multiple fields of a struct
    // Rust knows that .x and .y refer to different objects
    // this is not the case with arrays you'd have to do something
    // like .split_at_mut(index)
    let y = &mut p.y;

    *y += p.x;

    println!("{}, {}", p.x, p.y);
}
