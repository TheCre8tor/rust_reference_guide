#[path = "./struct.rs"]
mod structs;

#[path = "./impl_struct.rs"]
mod impl_struct;

#[path = "./1_arrays.rs"]
mod arrays;

mod impl_enums;
mod turples;

#[path = "./2_vectors.rs"]
mod vectors;

#[path = "./3_hashmap.rs"]
mod hashmap;

#[path = "./4_slices.rs"]
mod slices;

#[path = "./exercises/word_counter.rs"]
mod word_counter;

fn main() {
    // structs::run();
    // impl_struct::run();
    // impl_enums::run();
    // arrays::run();
    // turples::run();
    // vectors::run();
    // hashmap::run();
    // slices::run();
    word_counter::run();
}



