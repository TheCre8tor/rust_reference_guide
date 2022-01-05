// primitive_datatypes.rs

fn main() {
    /* NOTE: Rust Primitive Datatypes
       Rust has the following built-in primitive types:
       
       1. bool: -> These are the usual booleans and can be either true or false.

       2. char: -> Characters, such as e.

       3. Integer types: -> These are characterized by the bit width. Rust supports
          integers that are up to 128 bits wide:

       -----------------------------------------------
          Signed                    Unsigned
       -----------------------------------------------

          i8                        u8
       -----------------------------------------------
          i16                       u16
       -----------------------------------------------
          i32                       u32
       -----------------------------------------------
          i64                       u64
       -----------------------------------------------
          i128                      u128

       4. isize: -> The pointer-sized [signed integer type]. Equivalent to i32 on 
          32-bit CPU and i64 on 64-bit CPU.

       5. usize: -> The pointer-sized [unsigned integer type]. Equivalent to i32 
          on 32-bit CPU and i64 on 64-bit CPU.
        
       6. f32: -> The 32-bit floating point type. Implements the IEEE 754 standard 
          for floating point representation.

       7. f64: -> The 64-bit floating point type.

       8. [T; N]: -> A fixed-size array, for the element type, T , and the 
          non-negative compile-time constant size N.

       9. [N]: -> A dynamically-sized view into a contiguous sequence, for any type T.

       10. str: -> String slices, mainly used as a reference, that is, &str.
       
       11. (T, U, ..) : A finite sequence called Tuple, (T, U, ..) where T and U can 
           be different types.

       12. fn(i32) -> i32 : A function that takes an i32 and returns an i32. 
           ? Functions also have a type.
           
       */
}