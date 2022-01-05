// variables_and_immutability.rs

fn main() {
    /* NOTE: Declaring variables and immutability -->

       In mainstream imperative languages such as C or Python, initializing 
       a variable does not stop you from reassigning it to some other value.

       Rust deviates from the mainstream here by making variables immutable 
       by default, that is, you cannot assign the variable to some other 
       value after you have initialized it. 
       
       If you need a variable to point to something else (of the same type) 
       later, you need to put the [mut] keyword before it. Rust asks you to 
       be explicit about your intent as much as possible. 
       
       Consider the following code: 
     */

    let target = "World";
    let mut greeting = "Hello";

    println!("{} {}", target, greeting);

    greeting = "How are you?";
    // target = "mate"; // NOTE: this won't compile, cannot assign twice to immutable variable.

    println!("{} {}", target, greeting);

    /* NOTE: [let] does much more than assign variables. It is a 
       pattern-matching statement in Rust */
}