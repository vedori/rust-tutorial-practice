#![allow(unused, clippy::useless_vec)]
// 1)
mod collections_vector;
// 2)
mod get_vs_indexing;
// 3)
mod mutable_and_immutable_ref;
// 4)
mod iterating_over_vec_values;
// 5)
mod vector_of_enums;

fn main() {
    collections_vector::example();
    get_vs_indexing::example();
    mutable_and_immutable_ref::example();
    iterating_over_vec_values::example();
    vector_of_enums::example();
}
