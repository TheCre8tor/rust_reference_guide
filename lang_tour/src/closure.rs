// closures.rs

fn main() {
   /* NOTE: Closures -->
      Closures are like functions but have more information of 
      the environment or scope in which they are declared. While 
      functions have names associated with them, closures are 
      defined without a name, but they can be assigned to a 
      variable. 

      * 1. The major use case for closures are as parameters to higher-order functions.
      * 2. A higher-order function is a function that takes another function 
      *    or closure as its argument.
      
    */ 

   let doubler = |x: u8| x * 2 ;
   let value = 5;

   let twice = doubler(value);
   println!("{} doubled is {}", value, twice);

   let big_closure = |b: u8, c: u8| -> u8 {
      let z = b + c;
      z * twice
   };

   let some_number = big_closure(1, 2);
   println!("Result from closure: {:#?}", some_number);

}