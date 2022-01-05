// turple_struct.rs

fn main() {
    /* EXPLANATION: Turple Struct -->
       The second form of struct is the tuple struct, which 
       has associated data. Here, the individual fields are 
       not named, but are referred to by their position in 
       the definition. 
       
       The tuple struct is an ideal choice when you need to 
       model data that has less than four or five attributes. 
       Anything more than that hinders readability and reasoning. */

    struct Color(u8, u8, u8);

    let white = Color(255, 255, 255);

    // You can pull them out by index ->
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("RED: {}", red);
    println!("GREEN: {}", green);
    println!("BLUE: {}\n", blue);

    let orange = Color(255, 165, 0);

    // You can also destructure the fields directly
    let Color(r, g, b) = orange;
    println!("R: {} G: {} B: {} (orange)", r, g, b);

    // Can also ignore fields while destructuring
    let Color(r, _, b) = orange;

}