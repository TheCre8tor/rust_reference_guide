// unit_struct.rs

fn main() {
    /* EXPLANATION: Unit Structs --> 
       In Rust, there are three forms of structs that we 
       can declare. The simplest of them is the unit struct, 
       which is written with the struct keyword, followed by 
       its name and a semicolon at the end. 
       
       * We can initialize this type using only its name. value 
       * now contains an instance of Dummy and is a zero sized 
       * value. 
       
       * Unit structs do not take any size at runtime as they 
       * have no data associated with them. 
       
       There are very few use cases for unit structs: 
       1. They can be used to model entities with no data or 
          state associated with them.

       2. Another use case is to use them to represent error 
          types, where the struct itself is sufficient to understand 
          the error without needing a description of it.

       3. Another use case is to represent states in a state machine 
          implementation.
       */
       

    // Unit Structs -->
    #[derive(Debug)]
    struct Dummy;

    let value = Dummy;

    println!("{:?}", value);
}