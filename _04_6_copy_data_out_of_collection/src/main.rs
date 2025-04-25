#![allow(unused)]
/* Problem snippet
{
    v = vec![String::from("hi"); // owns String heap data
    v_ref = &mut v;
    v_val = *v_ref[0]. // owns String heap data
} // v and v_val are dropped here resulting in double free

 * The problem is that v_val also OWNS the hi String data.
 * When v and v_ref both go out of scope its a double free
 * This ONLY happens for values that OWN HEAP DATA
 * if it was &i32 this would be fine since it implements Copy
 *  unless it was &mut i32 since that could bring about
 *  a race condition from two references to the data mutating it at the same time
*/

// Avoid taking ownership of the string data by using an immutable reference
fn solution_1() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
}

// Clone the data if you want ownership
fn solution_2() {
    let v: Vec<String> = vec![String::from("Hello World")];
    let mut s = v[0].clone();
}

// Use methods that move string data out of the vector
fn solution_3() {
    let mut v: Vec<String> = vec![String::from("Hello World")];
    let mut s = v.remove(0);
}

fn main() {
    println!("Hello, world!");
}
