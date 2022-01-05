//! 1_arrays

pub fn run() {
    /* NOTE: Arrays have a fixed length that can store
    items of the same type. They are denoted by [T, N],
    where T is any type and N is the number of elements
    in array. */

    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    let floats: [f32; 5] = [0.3, 8.4, 2.9, 34.9, 9.0];
    let float_check = [0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[4]);
    println!("Float: {}", float_check[0]);
}
