// loops.rs

#[path = "./loop_labels.rs"]
mod loops;

fn main() {
    /* NOTE: Loops -->
       Repeating things in Rust can be done using three constructs, 
       namely loop , while , and for . In all of them, we have the 
       usual continue and break keywords, which allow you to skip 
       and break out of a loop, respectively. 
       
     */

    let mut x = 100;

    loop {
        if x < 0 {
            break;
        }

        println!("{} more runs to go", x);
        x -= 1;
    }
}