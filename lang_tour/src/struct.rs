// struct.rs

pub fn run() {
    let name: String = "Alice".to_string();

    let player = Player {
        name,
        iq: 171,
        friends: 134,
        score: 1129,
    };

    bump_player_score(player, 120);
}

/* NOTE: The advantage of using a struct rather than a tuple
struct is that we can initialize the fields in any order.
It also allows us to provide meaningful names to the fields.

As a side note, the size of a struct is simply the sum of
its individual field members, along with any data alignment
padding, if required.

They don't have any extra metadata size overhead associated
with them. */

struct Player {
    name: String,
    iq: u8,
    friends: u8,
    score: u16,
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += score;

    println!("Updated player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friends);
    println!("Score: {}", player.score);
}
