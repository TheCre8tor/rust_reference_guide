// strings.rs

fn main() {
    /* EXPLANATION: Strings
       Strings are one of the most frequently used data types in
       any programming language. In Rust, they are usually found
       in two forms: the [&str] and [String] type.__rust_force_expr!
       
       Rust strings are guaranteed to be valid UTF-8 encoded byte
       sequences. They are not null terminated as in C strings and
       can contain null bytes in-between them.
       
     */

    let question: &str = "How are you?";  // a &str type -->
    let person: String = "Bob".to_string();
    let namaste = String::from("U+0665"); // unicode

    println!("{}! {} {}", namaste, question, person);

    /* NOTE: Strings are allocated on the heap, while &str types are 
       usually pointers to an existing string, which could either be 
       on stack, the heap, or a string in the data segment of the 
       compiled object code. 

       * The & is an operator that is used to create a pointer to any type.
    
       */
}