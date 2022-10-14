pub mod field;
pub mod player;
pub mod position;

use crate::{field::Field, player::Player, position::Position};

fn action(
    field: &Field,
    position_row_number: usize,
    new_position_row_number: usize,
    position: &Position,
    new_position: &Position,
    current_player: &Player,
) {
    let position_value: String = field.field[position_row_number][position.column - 1].clone();
    let new_position_value: String =
        field.field[new_position_row_number][new_position.column - 1].clone();

    if current_player.piece_type != position_value {
        panic!("Cannot change a position");
    }

    if new_position_value == ".....".to_string() {
        println!("Move");
    } else if new_position_value != current_player.piece_type {
        println!("Eat");
    }
}

pub fn run(player_one: Player, player_two: Player, field: Field) {
    let mut current_player: &Player = &player_one;
    let mut rounds: usize = 0;

    loop {
        println!("\nOkay! This is the checkers states:");
        field.show();

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
        let position: Position = Position::new();

        println!("\nWhere do you wanna put? (Ex: C2)?");
        let new_position: Position = Position::new();

        let piece_row_number: usize = position.convert_row_index();
        let position_row_number: usize = new_position.convert_row_index();

        println!(
            "\nGet {} and move to {}",
            field.field[piece_row_number][position.column - 1],
            field.field[position_row_number][new_position.column - 1],
        );

        rounds += 1;
        break;
    }
}
