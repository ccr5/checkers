pub mod field;
pub mod player;
pub mod position;

use crate::{field::Field, player::Player, position::Position};

pub struct Checkers {
    pub player_one: Player,
    pub player_two: Player,
    pub field: Field,
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

        Checkers {
            player_one,
            player_two,
            field,
        }
    }

    pub fn run(&self) {
        let mut current_player: &Player = &self.player_one;
        let mut rounds: usize = 0;

        loop {
            println!("\nOkay! This is the checkers states:");
            self.field.show();

            if rounds == 0 {
                println!("The first player to begin is {}", self.player_one.name);
            } else {
                if rounds % 2 != 0 {
                    current_player = &self.player_two;
                }

                println!("The next player to begin is {}", current_player.name);
            }

            println!("What piece do you want to move (Ex: C2)?");
            let position: Position = Position::new();

            println!("\nWhere do you wanna put? (Ex: C2)?");
            let new_position: Position = Position::new();

            self.action(&position, &new_position, &current_player);

            rounds += 1;
        }
    }

    pub fn action(&self, position: &Position, new_position: &Position, current_player: &Player) {
        let position_row_number: usize = position.convert_row_index();
        let new_position_row_number: usize = new_position.convert_row_index();

        println!(
            "\nGet {} and move to {}",
            self.field.field[position_row_number][position.column - 1],
            self.field.field[new_position_row_number][new_position.column - 1],
        );

        let position_value: &i8 = &self.field.field[position_row_number][position.column - 1];
        let new_position_value: &i8 =
            &self.field.field[new_position_row_number][new_position.column - 1];

        if current_player.piece_type.to_string() != position_value.to_string() {
            panic!("Cannot change a position");
        }

        if new_position_value.to_string() == ".....".to_string() {
            self.field
                .update(position_row_number, position.column - 1, -1);

            self.field.update(
                new_position_row_number,
                new_position.column - 1,
                current_player.piece_type,
            );

            println!("Move");
        } else if new_position_value != &current_player.piece_type {
            println!("Eat");
        }
    }
}
