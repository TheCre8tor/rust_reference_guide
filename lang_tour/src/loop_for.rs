fn main() {
    // Does not include 10
    println!("Normal Ranges: \n");
    for i in 1..10 {
        println!("{}", i);
    }

    println!();
    // print 1 -> 10
    println!("Inclusive Range: \n");
    for i in 1..=10 {
        println!("{}", i);
    }
}