//! turples.rs

pub fn run() {
    /* NOTE: Tuples differ from arrays in the way that
       elements of an array have to be of the same type,
       while items in a tuple can be a mix of types.
       They are heterogeneous collections and are useful
       for storing distinct types together. They can also
       be used when returning multiple values from a
       function. */

    let num_and_str: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", num_and_str);

    let (num, string) = num_and_str;
    println!("From turple: Number: {}, String: {}", num, string);
}