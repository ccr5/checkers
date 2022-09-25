mod field;
mod player;

use crate::field::field::{convert_row_index, create_field, show_field};
use crate::player::player::{create_player, Player};
use std::io;
use std::str::SplitWhitespace;

fn main() {
    println!("Welcome Ferris!");
    println!("This is my first Rust code ;)\n");

    println!("Please, who will play?");
    println!("--- Player 1 ---");
    let player_one: Player = create_player();

    println!("\nAll right. And who will be your opponent?");
    println!("--- Player 2 ---");
    let player_two: Player = create_player();

    let mut field: [[String; 8]; 8] = create_field(&player_one, &player_two);
    let mut next_player: &Player = &player_one;
    let mut rounds: i8 = 0;

    loop {
        let current_player = next_player.clone();

        println!("\nOkay! This is the checkers states:");
        show_field(&field);

        if rounds == 0 {
            println!("The first player to begin is {}", current_player);
            next_player = player_two.name.clone();
        } else {
            println!("The next player to begin is {}", current_player);
            if rounds % 2 == 0 {
                next_player = player_one.name.clone();
            } else {
                next_player = player_two.name.clone();
            }
        }

        rounds += 1;

        println!("What piece do you want to move (Ex: C 2)?");
        let mut piece: String = String::new();
        io::stdin()
            .read_line(&mut piece)
            .expect("Error: Cannot read line");

        println!("Where do you wanna put? (Ex: C 2)?");
        let mut position: String = String::new();
        io::stdin()
            .read_line(&mut position)
            .expect("Error: Cannot read line");

        piece.pop();
        position.pop();

        let mut split_piece: SplitWhitespace = piece.split_whitespace();
        let mut split_position: SplitWhitespace = position.split_whitespace();
        let piece_row: &str = split_piece.next().expect("Error");
        let piece_col: &str = split_piece.next().expect("Error");
        let position_row: &str = split_position.next().expect("Error");
        let position_col: &str = split_position.next().expect("Error");

        let piece_col_number: usize = piece_col.parse().expect("error");
        let piece_row_number = convert_row_index(&piece_row);
        let position_col_number: usize = position_col.parse().expect("error");
        let position_row_number = convert_row_index(&position_row);

        println!(
            "Get {} and move to {}",
            field[piece_row_number][piece_col_number - 1],
            field[position_row_number][position_col_number - 1],
        );

        println!("{}", next_player);

        if field[piece_row_number][piece_col_number - 1] == ".....".to_string() {
            println!("Move");
        } else if field[piece_row_number][piece_col_number - 1] == next_player.to_string() {
            println!("Eat");
        }

        break;
    }

    // Create Checkers functions like play, check who wins
    // Start loops
}
