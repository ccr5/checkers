pub mod field {

    use crate::player::player::Player;

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
}
