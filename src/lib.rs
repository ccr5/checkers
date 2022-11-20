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

    fn create_checker() -> Checkers {
        let player_one: Player = Player {
            name: "Matheus".to_string(),
            piece_type: 1,
        };

        let player_two: Player = Player {
            name: "Lucas".to_string(),
            piece_type: 0,
        };

        let field: Field = Field::new(&player_one, &player_two);

        Checkers {
            player_one,
            player_two,
            field,
            rounds: 0,
        }
    }

    #[test]
    fn test_current_player() {
        let mut checkers = create_checker();
        let mut current_player = checkers.get_current_player();
        assert_eq!(current_player, checkers.player_one);
        checkers.rounds += 1;
        current_player = checkers.get_current_player();
        assert_eq!(current_player, checkers.player_two);
    }

    #[test]
    fn test_validate() {
        let mut checkers = create_checker();

        let field_one: Field = Field {
            field: [
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, 1, -1, -1],
            ],
        };

        let field_two: Field = Field {
            field: [
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, 0, -1, -1],
            ],
        };

        let field_three: Field = Field {
            field: [
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, 1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, 0, -1, -1],
            ],
        };

        checkers.field = field_one;
        let (winner_one, winner_name_one) = checkers.validate();

        checkers.field = field_two;
        let (winner_two, winner_name_two) = checkers.validate();

        checkers.field = field_three;
        let (winner_three, winner_name_three) = checkers.validate();

        assert_eq!(winner_one, true);
        assert_eq!(winner_name_one, checkers.player_one.name);
        assert_eq!(winner_two, true);
        assert_eq!(winner_name_two, checkers.player_two.name);
        assert_eq!(winner_three, false);
        assert_eq!(winner_name_three, "".to_string());
    }

    #[test]
    fn test_action() {
        let mut checkers: Checkers = create_checker();
        let current_player: Player = checkers.get_current_player();
        let position: Position = Position {
            column: 1,
            row: "C".to_string(),
        };
        let new_position: Position = Position {
            column: 2,
            row: "D".to_string(),
        };

        let action_result = match checkers.action(&position, &new_position, &current_player) {
            Ok(()) => true,
            Err(err) => {
                println!("{}", err);
                false
            }
        };

        let field_test: Field = Field {
            field: [
                [-1, 1, -1, 1, -1, 1, -1, 1],
                [1, -1, 1, -1, 1, -1, 1, -1],
                [-1, -1, -1, 1, -1, 1, -1, 1],
                [-1, -1, 1, -1, -1, -1, -1, -1],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [0, -1, 0, -1, 0, -1, 0, -1],
                [-1, 0, -1, 0, -1, 0, -1, 0],
                [0, -1, 0, -1, 0, -1, 0, -1],
            ],
        };

        assert_eq!(action_result, true);
        assert_eq!(checkers.field, field_test);
    }
}
