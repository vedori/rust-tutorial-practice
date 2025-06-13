// Types that inherit the Copy trait like i32 get their values copied into the hash map
// However the types that don't will be **moved** and the hash map will be the owner
// of those values

use std::collections::HashMap;

fn ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are now invalid
    // The statement below would fail to compile
    // println!("{field_name} {field_value}");

    // Given this, references are often passed to Hashmaps
    // When dealing with references, lifetimes become important
    // The valies that the references point to must be valid for at least as long
    // as the hash map is valid (more on this in Chapter 10)
}
