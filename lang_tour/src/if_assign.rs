// if_assign.rs

fn main() {
    /* EXPLANATION: Conditional and Decision Making -->
       In Rust, the if construct is not a statement, but an expression.
       
       In general programming parlance, statements do not return any value, 
       but an expression does.

       This distinction means that if else conditionals in Rust always 
       return a value.

       NOTE: The value may be an empty () unit type, or it may be an actual 
       value. Whatever remains in the last line inside the braces becomes 
       the return value of the if else expression.

       It is important to note that both if and else branches should have 
       the same return type.
       */

    let rust_is_awesome = false;

    if rust_is_awesome {
        println!("Indeed");
    } else {
        println!("Well, you should try Rust!");
    }

    // #-->
    let result = if 1 == 2 {
        "What, what?"
    } else {
        "Rust makes sense"
    };

    println!("You know what? {}.", result);
}