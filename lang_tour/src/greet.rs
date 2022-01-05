// greet.rs

use std::env;

fn main() {
    /* args() method -> Returns the arguments that this program 
       was started with (normally passed via the command line). 
       
       NOTE: Libary's are called crates -->

       EXPLANATION: Program explanation ->
       1. We call the function args() from the env module, which returns 
       an iterator (sequence) of arguments that has been passed to our 
       program.

       2. Since the first argument contains our program name, we want to 
       skip it, so wecall skip and pass in a number, which is how many 
       elements (1) we want to skip.

       3. As iterators are lazy and do not pre-compute things in Rust, we 
       have to explicitly ask it to give the next element, so we call 
       next() , which returns an enum type called Option . This can be 
       either a Some(value) value or a None value because a user might 
       forget to provide an argument.

       4. We use Rust's awesome match expression on the variable name and 
       check whether it's a Some(n) or a None value. 
       * [match] is like the if else construct, but more powerful.
       
       5.
       
     */

    let name = env::args().skip(1).next();

    println!("{:?}", name);

    match name {
        Some(n) => println!("Hi there! {}", n),
        None => panic!("Didn't receive any name ?"),
    };
}