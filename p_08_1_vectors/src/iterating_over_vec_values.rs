pub fn example() {
    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        // needs to be dereferenced to get the value in n_ref
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }
}
