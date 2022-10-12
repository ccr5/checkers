pub mod field {

    use checkers::Player;
    use std::io;
    use std::str;

    #[derive(Debug)]
    pub struct Position {
        pub column: usize,
        pub row: String,
    }

    pub fn create_field(player_one: &Player, player_two: &Player) -> [[String; 8]; 8] {
        let field: [[String; 8]; 8] = [
            [
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
            ],
            [
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
            ],
            [
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
                ".....".to_string(),
                player_one.piece_type.clone(),
            ],
            [
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
            ],
            [
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
                ".....".to_string(),
            ],
            [
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
            ],
            [
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
            ],
            [
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
                player_two.piece_type.clone(),
                ".....".to_string(),
            ],
        ];

        return field;
    }

    pub fn show_field(field: &[[String; 8]; 8]) {
        let cols: [String; 8] = [
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
            "H".to_string(),
        ];
        println!("        1        2        3        4        5        6        7        8     ");
        for index in 0..field.len() {
            println!("{} - {:?}", cols[index], field[index]);
        }
    }

    pub fn get_position() -> Position {
        let mut position_text: String = String::new();
        let position: Position = match io::stdin().read_line(&mut position_text) {
            Ok(_) => {
                let position_row: char = position_text
                    .chars()
                    .nth(0)
                    .expect("Cannot get the column value");

                let position_column: char = position_text
                    .chars()
                    .nth(1)
                    .expect("Cannot get the row value");

                Position {
                    column: position_column
                        .to_string()
                        .parse()
                        .expect("Cannot parse the value"),
                    row: position_row.to_string(),
                }
            }
            Err(_) => panic!(),
        };

        return position;
    }

    pub fn convert_row_index(col: &str) -> usize {
        let cols: [String; 8] = [
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
            "G".to_string(),
            "H".to_string(),
        ];

        return cols.iter().position(|x| x == col).unwrap();
    }

    pub fn action(
        mut field: &[[String; 8]; 8],
        position_row_number: usize,
        new_position_row_number: usize,
        position: &Position,
        new_position: &Position,
        current_player: &Player,
    ) {
        let position_value: String = field[position_row_number][position.column - 1].clone();
        let new_position_value: String =
            field[new_position_row_number][new_position.column - 1].clone();

        if current_player.piece_type != position_value {
            panic!("Cannot change a position");
        }

        if new_position_value == ".....".to_string() {
            println!("Move");
        } else if new_position_value != current_player.piece_type {
            println!("Eat");
        }
    }
}
