pub fn run() {
    /* LEC: Implement Blocks on Structs -->
       We can add behavior to our previously defined Player struct
       with two functionalities: a constructor-like function that
       takes a name and sets default values for the remaining fields
       in Person , and getter and setter methods for the friend
       count of Person :

    */

    struct Player {
        name: String,
        iq: u8,
        friends: u8,
    }

    impl Player {
        /* EXPLANATION: Associated Methods -->
           Methods without a self type as their first parameter.
           The with_name method is called an associated method because
           it does not have self as the first parameter.

           * It is similar to a static method in object-oriented languages.

           ? These methods are available on the type themselves and do not
           ? need an instance of the type to invoke them.
        */

        fn with_name(name: &str) -> Player {
            Player {
                name: name.to_string(),
                iq: 100,
                friends: 100,
            }
        }

        /* EXPLANATION: Instance Methods -->
        Functions that take a self value as its first argument.
        Therefore, this method can only be called on already created
        instances of the struct. */

        fn get_friends(&self) -> u8 {
            self.friends
        }

        fn set_friends(&mut self, count: u8) {
            self.friends = count;
        }
    }

    let mut player = Player::with_name("Alexander");
    player.set_friends(23);

    println!("{}'s friends count: {}", player.name, player.get_friends());

    // ? Another way to call an instance methods.

    let _ = Player::get_friends(&player);
}
