// match_expression.rs

fn main() {
    /* NOTE: Rust's match expressions is basically C's 
       switch statement on steroids and allows you to make 
       decisions, depending on what value the variable has 
       and whether it has advanced filtering capabilities. 

       * Within braces, we write expressions – these are 
       * alled match arms.
       
       * Every match arm must return the same type.
       
       Like if else expressions, the return value of a match 
       expression can also be assigned to a variable in a let 
       statement when it's delimited with a semicolon, with 
       all match arms returning the same types.
    */ 

    let status = req_status();

    match status {
        200 => println!("Success"),
        404 => println!("Not found!"),
        other => {
            println!("Request failed with code: {}", other);
            // get response from cache
        }
    }

}

fn req_status() -> u32 {
    401   
}