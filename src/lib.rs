pub mod field;
pub mod player;
pub mod position;

use crate::{field::Field, player::Player, position::Position};
use colored::Colorize;

pub struct Checkers {}

impl Checkers {
    pub fn new() -> (Player, Player, Field) {
        println!("Welcome Ferris!");
        println!("This is my first Rust code ;)\n");

        println!("Please, who will play?");
        println!("--- Player 1 ---");
        let player_one: Player = Player::new();

        println!("\nAll right. And who will be your opponent?");
        println!("--- Player 2 ---");
        let player_two: Player = Player::new();

        let field: Field = Field::new(&player_one, &player_two);

        (player_one, player_two, field)
    }

    pub fn run(&self, player_one: &mut Player, player_two: &mut Player, field: &mut Field) {
        let mut current_player: &Player = &player_one;
        let mut rounds: usize = 0;

        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!("\nOkay! This is the checkers states:");
            field.show();

            if rounds == 0 {
                println!("The first player to begin is {}", player_one.name.bold());
            } else {
                if rounds % 2 != 0 {
                    current_player = &player_two;
                }

                println!("The next player to begin is {}", current_player.name.bold());
            }

            let mut result: bool = false;

            while !result {
                println!("What piece do you want to move (Ex: C2)?");
                let position: Position = Position::new();

                println!("\nWhere do you wanna put? (Ex: C2)?");
                let new_position: Position = Position::new();

                result = match self.action(&position, &new_position, &current_player, field) {
                    Ok(()) => true,
                    Err(err) => {
                        println!("{}", err);
                        println!("Please, play again");
                        false
                    }
                };
            }

            rounds += 1;
        }
    }

    pub fn action(
        &self,
        position: &Position,
        new_position: &Position,
        current_player: &Player,
        field: &mut Field,
    ) -> Result<(), &str> {
        let position_row_number: usize = position.convert_row_index();
        let new_position_row_number: usize = new_position.convert_row_index();
        let position_value: &i8 = &field.get(position_row_number, position.column);
        let new_position_value: &i8 = &field.get(new_position_row_number, new_position.column);
        let check_row = {
            if new_position_row_number > position_row_number {
                new_position_row_number - position_row_number
            } else {
                position_row_number - new_position_row_number
            }
        };

        let check_col = {
            if new_position.column > position.column {
                new_position.column - position.column
            } else {
                position.column - new_position.column
            }
        };

        if current_player.piece_type.to_string() != position_value.to_string() {
            return Err("Mismatch piece. Choose your piece.");
        } else if check_row != 1 || check_col != 1 {
            return Err("You ONLY can move in diagonal");
        }

        if new_position_value.to_string() == "-1" {
            field.update(position_row_number, position.column, -1);
            field.update(
                new_position_row_number,
                new_position.column,
                current_player.piece_type,
            );

            println!("Move");
        } else if new_position_value != &current_player.piece_type {
            field.update(position_row_number, position.column, -1);
            field.update(
                new_position_row_number,
                new_position.column,
                current_player.piece_type,
            );
            println!("Eat");
        }

        Ok(())
    }
}
