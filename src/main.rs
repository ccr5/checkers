mod field;
mod player;

use crate::field::field::{create_field, show_field};
use crate::player::player::{create_player, Player};

fn main() {
    println!("Welcome Ferris!");
    println!("This is my first Rust code ;)\n");
    println!("Please, who will play?");

    println!("--- Player 1 ---");
    let player_one: Player = create_player();

    println!("\nAll right. And who will be your opponent?");
    println!("--- Player 2 ---");
    let player_two: Player = create_player();

    println!("\nOkay! This is the checkers states:");
    let mut field: [[String; 8]; 8] = create_field(player_one, player_two);
    show_field(field);

    // Create Checkers functions like play, check who wins
    // Start loops
}
