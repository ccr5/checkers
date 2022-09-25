mod field;
mod player;

use crate::field::field::{create_field, show_field};
use crate::player::player::{create_player, Player};
use std::io;

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
    let mut field: [[String; 8]; 8] = create_field(&player_one, &player_two);
    show_field(field);
    println!("let's start!");

    let mut next_player: &String = &player_one.name;
    let mut rounds: i8 = 0;

    loop {
        if rounds == 0 {
            println!("The first player to begin is {}", next_player);
            next_player = &player_two.name;
        } else {
            println!("The next player to begin is {}", next_player);
            if rounds % 2 == 0 {
                next_player = &player_one.name;
            } else {
                next_player = &player_two.name;
            }
        }

        rounds += 1;

        println!("What piece do you want to move?");
        let mut piece: String = String::new();
        io::stdin()
            .read_line(&mut piece)
            .expect("Error: Cannot read line");
    }

    // Create Checkers functions like play, check who wins
    // Start loops
}
