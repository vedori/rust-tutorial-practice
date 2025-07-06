// == Key problem ==
// This function gets a reference to the largest string in a vector
// It then attempts to use this reference while mutating the vector
// =================

// This function gets a reference to the biggest string in the `dst` vector
// Then it compares the length of the largest string to all the strings in `src` String slice
// If any string in `src` is bigger than that string reference it is pushed to the `dst` vector
fn bad_example_add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    // L1
    // == L1 ==
    // *dst: RW
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // L2
    // == L2 ==
    // *dst: R(W perms removed since `largest` immutably borrows data at dst)
    // This happens because mutating the contents of dst invalidates the `largest` reference
    // This behavior is covered in 4.2 immutable_vs_mutable_references
    for s in src {
        if s.len() > largest.len() {
            // As a result, *dst CANNOT be mutated until the lifetime of `largest` ended
            // push() requires W perms
            todo!("dst.push(s.clone())");
        }
    }
}

// It is clear that the way to fix this program is to shorten the lifetime of `largest`
// in such a way that it does not overlap with `dst.push(...)`

// One way is to clone largest
// This is suboptimal because cloning could be mean a performance hit given the necessary
// allocation and copying of string data
fn suboptimal_solution_1(dst: &mut Vec<String>, src: &[String]) {
    let larget = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > larget.len() {
            dst.push(s.clone());
        }
    }
}

// Another solution is to perform all the length comparisons first
// then mutate `dst` afterwards
// This solution as written is suboptimal because of the need to allocate the `to_add` vector
fn suboptimal_solution_2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    // Lifetime to largest ended after call to filter()
    let to_add: Vec<String> = src
        .iter()
        .filter(|s| s.len() > largest.len())
        .cloned()
        .collect();
    dst.extend(to_add);
}

// This is the best solution
// Copy the length of the string because we dont need the string data itself
// Then use that length of type usize to figure out which strings from src
// to add to dst
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    // Lifetime of the largest string in dst ended after the call to len()
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
