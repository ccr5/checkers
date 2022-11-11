pub mod field;
pub mod player;
pub mod position;

use crate::{field::Field, player::Player, position::Position};
use colored::Colorize;
use std::error::Error;

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

    pub fn run(
        &self,
        player_one: &mut Player,
        player_two: &mut Player,
        field: &mut Field,
    ) -> Result<(), Box<dyn Error>> {
        let mut current_player: &Player = &player_one;
        let mut rounds: usize = 0;
        let mut winner: bool = false;

        loop {
            if winner {
                break;
            }

            print!("\x1B[2J\x1B[1;1H");
            println!("\nOkay! This is the checkers states:");
            field.show();

            if rounds == 0 {
                println!("The first player to begin is {}", player_one.name.bold());
            } else {
                if rounds % 2 != 0 {
                    current_player = &player_two;
                } else {
                    current_player = &player_one;
                }

                println!("The next player to begin is {}", current_player.name.bold());
            }

            let mut action_result: bool = false;

            while !action_result {
                println!("What piece do you want to move (Ex: C2)?");
                let position: Position = Position::new();

                println!("\nWhere do you wanna put? (Ex: C2)?");
                let new_position: Position = Position::new();

                action_result = match self.action(&position, &new_position, &current_player, field)
                {
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

        println!("\nThe winner is: {}", winner);
        Ok(())
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

    pub fn validate(
        &self,
        player_one: Player,
        player_two: Player,
        field: Field,
    ) -> Result<i8, Box<dyn Error>> {
        let mut has_player_one: bool = false;
        let mut has_player_two: bool = false;

        // Loop validando

        if has_player_one && has_player_two {
            Ok(-1)
        } else if has_player_one && !has_player_two {
            Ok(player_one.piece_type)
        } else if !has_player_one && has_player_two {
            Ok(player_two.piece_type)
        } else {
            panic!("Invalid condition")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{field::Field, player::Player, position::Position, Checkers};

    #[test]
    fn test_action() {
        let checkers: Checkers = Checkers {};
        let (_, current_player, mut field) = Checkers::new();
        let position: Position = Position {
            column: 2,
            row: "C".to_string(),
        };
        let new_position: Position = Position {
            column: 3,
            row: "D".to_string(),
        };

        let action_result =
            match checkers.action(&position, &new_position, &current_player, &mut field) {
                Ok(()) => true,
                Err(err) => {
                    println!("{}", err);
                    println!("Please, play again");
                    false
                }
            };

        let position_result: i8 = field.get(2, 1);
        let new_position_result: i8 = field.get(3, 2);

        assert_eq!(true, action_result);
        assert_eq!(1, position_result);
        assert_eq!(-1, new_position_result);
    }

    #[test]
    fn test_validate() {
        let checkers: Checkers = Checkers {};
        let player_one: Player = Player {
            name: "Matheus".to_string(),
            piece_type: 1,
            matchs: 0,
            victories: 0,
        };

        let player_two: Player = Player {
            name: "Lucas".to_string(),
            piece_type: 0,
            matchs: 0,
            victories: 0,
        };

        let field_test: [[i8; 8]; 8] = [
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, 0, -1, -1],
        ];

        let field: Field = Field { field: field_test };
        let result: i8 = match checkers.validate(player_one, player_two, field) {
            Ok(value) => value,
            Err(_err) => panic!("Invalid condition"),
        };

        assert_eq!(result, 0);
    }
}
