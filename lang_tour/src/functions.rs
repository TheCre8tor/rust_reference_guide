fn main() {
    /* EXPLANATION: Functions abstract a bunch of instructions 
       into named entities, which can be invoked later by other 
       code and help manage complexity. 
       
       If you look at the body of add, we don't need a return 
       keyword to return a + b as in other languages. The last 
       expression is returned automatically. However, we do have 
       the return keyword available for early returns. 

       Functions are basically expressions that return a value, 
       which is a () (Unit) type by default, akin to the void 
       return type in C/C++.
    
       */

    let left: u8 = 3;
    let right: u8 = 5;

    let result = add(left, right);

    println!("{}", result);


    // Function that modify their arguments -->
    let score: u32 = 2048;

    increase_by(score, 30);

    println!("Does score change from 2048 to: {}", score);

}

fn add(left: u8, right: u8) -> u8 {
    left + right
}

fn increase_by(mut val: u32, how_much: u32) {
    /* mut val , indicating that the parameter should 
       be taken as mutable, which allows the variable 
       to be mutated from inside the function. 
     */

    val += how_much;
    println!("You made {} points", val);
}