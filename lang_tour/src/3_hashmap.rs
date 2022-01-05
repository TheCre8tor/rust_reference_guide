use std::collections::{HashMap};

pub fn run() {
    /* Rust also provides us with maps, which can be used
       to store key-value data. They come from the std::collections
       module and are named HashMap . They are created with the
       HashMap::new constructor function: */

    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k)
    }

    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\n now have {} avocados", fruits["avocado"]);
}