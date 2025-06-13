#![allow(unused)]

// 1)
mod hash_maps;

// 2)
mod ownership;

// 3)
mod updating_a_hash_map;

/*
 * By default `HashMap` uses a hash function called SipHash
 * It provides resistance to denial-of-service attacks involving hash tables
 * but at the cost of some performance.
 *
 * To change the hasher, you can set it to a type that implements the BuilderHasher trait
 * You dont have to implement your own hasher
 * since hash implementations can be found on crates.io
 */
