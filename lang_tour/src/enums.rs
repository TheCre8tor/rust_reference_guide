//! enums.rs

fn main() {
    /* EXPLANATION: ENUMS --> 
       When you need to model something that can be of different 
       kinds, enums are the way to go. 
       
       * They are created using the enum keyword, followed by the name 
       * of the enum, followed by a pair of braces. Within braces, we 
       * can write all the possibilities of the type, which are called 
       * variants.
       
       These variants can be defined with or without data contained in 
       them, and the data contained can be any primitive type, structs, 
       tuple structs, or even an enum.

       ? However, in the recursive case, where you have an enum, Foo ,and 
       ? also a variant which holds Foo , the variant needs to be behind a 
       ? pointer ( Box , Rc , and so on) type to avoid having recursively
       ? infinite type definitions.

       Because enums can also be created on the stack, they need to have 
       a predetermined size, and infinite type definitions makes it
       impossible to determine the size at compile time.

       NOTE: From a functional programmer's perspective, structs and enums 
       are also known as Algebraic Data Types (ADTs) because the possible 
       range of values they can represent can be expressed using the rules 
       of algebra.

       */

    
    // let simulated_player_action = PlayerAction::Move {
    //     direction: Direction::N,
    //     speed: 2,
    // };

    #[derive(Debug)]
    enum Direction {
        N,
        E,
        S,
        W,
    }

    #[derive(Debug)]
    enum PlayerAction {
        Move {
            direction: Direction,
            speed: u8,
        },
        Wait,
        Attack(Direction),
    }

    let _simulated_player_action = PlayerAction::Attack(Direction::N);
    let player_action = PlayerAction::Move { direction: Direction::W, speed: 10 };
    
    match player_action {
        PlayerAction::Wait => println!("Player wants to wait"),
        PlayerAction::Move { direction, speed } => {
            println!("Player wants to move in direction {:?} with speed {}", direction, speed);
        },
        PlayerAction::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction)
        }
    };
}