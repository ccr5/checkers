mod field;

use crate::field::field::*;
use checkers::Player;

fn main() {
    println!("Welcome Ferris!");
    println!("This is my first Rust code ;)\n");

    println!("Please, who will play?");
    println!("--- Player 1 ---");
    let player_one: Player = Player::new();

    println!("\nAll right. And who will be your opponent?");
    println!("--- Player 2 ---");
    let player_two: Player = Player::new();

    let field: [[String; 8]; 8] = create_field(&player_one, &player_two);
    let mut current_player: &Player = &player_one;
    let mut rounds: usize = 0;

    loop {
        println!("\nOkay! This is the checkers states:");
        show_field(&field);

        if rounds == 0 {
            println!("The first player to begin is {}", player_one.name);
        } else {
            if rounds % 2 == 0 {
                current_player = &player_one;
            } else {
                current_player = &player_two;
            }

            println!("The next player to begin is {}", current_player.name);
        }

        println!("What piece do you want to move (Ex: C2)?");
        let position: Position = get_position();

        println!("\nWhere do you wanna put? (Ex: C2)?");
        let new_position: Position = get_position();

        let piece_row_number: usize = convert_row_index(&position.row);
        let position_row_number: usize = convert_row_index(&new_position.row);

        println!(
            "\nGet {} and move to {}",
            field[piece_row_number][position.column - 1],
            field[position_row_number][new_position.column - 1],
        );

        action(
            &field,
            piece_row_number,
            position_row_number,
            &position,
            &new_position,
            &current_player,
        );

        rounds += 1;
        break;
    }
}
