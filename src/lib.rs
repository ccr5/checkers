pub mod field;
pub mod player;
pub mod position;

use crate::{field::Field, player::Player, position::Position};
use colored::Colorize;
use std::error::Error;

pub struct Checkers {
    player_one: Player,
    player_two: Player,
    field: Field,
    rounds: i32,
}

impl Checkers {
    pub fn new() -> Self {
        println!("Welcome Ferris!");
        println!("This is my first Rust code ;)\n");

        println!("Please, who will play?");
        println!("--- Player 1 ---");
        let player_one: Player = Player::new();

        println!("\nAll right. And who will be your opponent?");
        println!("--- Player 2 ---");
        let player_two: Player = Player::new();

        let field: Field = Field::new(&player_one, &player_two);

        Self {
            player_one,
            player_two,
            field,
            rounds: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut winner: bool = false;
        let mut winner_name: String = String::from("");

        loop {
            if winner {
                break;
            }

            print!("\x1B[2J\x1B[1;1H");
            println!("\nOkay! This is the checkers states:");
            self.field.show();
            let current_player: Player = self.get_current_player();
            let mut action_result: bool = false;

            while !action_result {
                println!("What piece do you want to move (Ex: C2)?");
                let position: Position = Position::new();

                println!("\nWhere do you wanna put? (Ex: C2)?");
                let new_position: Position = Position::new();

                action_result = match self.action(&position, &new_position, &current_player) {
                    Ok(()) => true,
                    Err(err) => {
                        println!("{}", err);
                        println!("Please, play again");
                        false
                    }
                };
            }

            (winner, winner_name) = self.validate();

            self.rounds += 1;
        }

        println!("\nThe winner is: {}", winner_name);
        Ok(())
    }

    fn get_current_player(&self) -> Player {
        if self.rounds == 0 {
            println!(
                "The first player to begin is {}",
                self.player_one.name.bold()
            );

            self.player_one.clone()
        } else {
            if self.rounds % 2 != 0 {
                println!(
                    "The next player to begin is {}",
                    &self.player_two.name.bold()
                );

                self.player_two.clone()
            } else {
                println!(
                    "The next player to begin is {}",
                    &self.player_one.name.bold()
                );

                self.player_one.clone()
            }
        }
    }

    fn action(
        &mut self,
        position: &Position,
        new_position: &Position,
        current_player: &Player,
    ) -> Result<(), &str> {
        let position_row_number: usize = position.convert_row_index();
        let new_position_row_number: usize = new_position.convert_row_index();
        let position_value: &i8 = &self.field.get(position_row_number, position.column);
        let new_position_value: &i8 = &self.field.get(new_position_row_number, new_position.column);
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
            self.field.update(position_row_number, position.column, -1);
            self.field.update(
                new_position_row_number,
                new_position.column,
                current_player.piece_type,
            );
        } else if new_position_value != &current_player.piece_type {
            self.field.update(position_row_number, position.column, -1);
            self.field.update(
                new_position_row_number,
                new_position.column,
                current_player.piece_type,
            );
        }

        Ok(())
    }

    fn validate(&self) -> (bool, String) {
        let mut has_player_one: bool = false;
        let mut has_player_two: bool = false;
        let default: String = String::from("");

        for row in self.field.field {
            for value in row {
                if value == self.player_one.piece_type {
                    has_player_one = true;
                } else if value == self.player_two.piece_type {
                    has_player_two = true;
                }
            }
        }

        if has_player_one && !has_player_two {
            (true, self.player_one.name.clone())
        } else if !has_player_one && has_player_two {
            (true, self.player_two.name.clone())
        } else {
            (false, default)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{field::Field, player::Player, position::Position, Checkers};

    #[test]
    fn test_action() {
        let player_one: Player = Player {
            name: "Matheus".to_string(),
            piece_type: 1,
        };

        let player_two: Player = Player {
            name: "Lucas".to_string(),
            piece_type: 0,
        };

        let field: Field = Field::new(&player_one, &player_two);

        let mut checkers: Checkers = Checkers {
            player_one,
            player_two,
            field,
            rounds: 0,
        };

        let position: Position = Position {
            column: 2,
            row: "C".to_string(),
        };
        let new_position: Position = Position {
            column: 3,
            row: "D".to_string(),
        };

        let current_player: Player = checkers.player_one.clone();

        let action_result = match checkers.action(&position, &new_position, &current_player) {
            Ok(()) => true,
            Err(_) => false,
        };

        let position_result: i8 = field.get(2, 1);
        let new_position_result: i8 = field.get(3, 2);

        assert_eq!(action_result, true);
        assert_eq!(position_result, -1);
        assert_eq!(new_position_result, 1);
    }

    // #[test]
    // fn test_validate() {
    //     let checkers: Checkers = Checkers {};
    //     let player_one: Player = Player {
    //         name: "Matheus".to_string(),
    //         piece_type: 1,
    //     };

    //     let player_two: Player = Player {
    //         name: "Lucas".to_string(),
    //         piece_type: 0,
    //     };

    //     let field_one: Field = Field {
    //         field: [
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, 0, -1, -1],
    //         ],
    //     };

    //     let field_two: Field = Field {
    //         field: [
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, 1, -1, -1],
    //         ],
    //     };

    //     let field_three: Field = Field {
    //         field: [
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, 1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, -1, -1, -1],
    //             [-1, -1, -1, -1, -1, 0, -1, -1],
    //         ],
    //     };

    //     let result_one: i8 = match checkers.validate(&player_one, &player_two, &field_one) {
    //         Ok(value) => value,
    //         Err(_err) => _err,
    //     };

    //     let result_two: i8 = match checkers.validate(&player_one, &player_two, &field_two) {
    //         Ok(value) => value,
    //         Err(_err) => _err,
    //     };

    //     let result_three: i8 = match checkers.validate(&player_one, &player_two, &field_three) {
    //         Ok(value) => value,
    //         Err(_err) => _err,
    //     };

    //     assert_eq!(result_one, 0);
    //     assert_eq!(result_two, 1);
    //     assert_eq!(result_three, -1);
    // }
}
