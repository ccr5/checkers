use crate::player::Player;

#[derive(Debug)]
pub struct Field {
    pub field: [[String; 8]; 8],
}

impl Field {
    pub fn new(player_one: &Player, player_two: &Player) -> Self {
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

        Field { field }
    }

    pub fn show(&self) {
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
        for index in 0..self.field.len() {
            println!("{} - {:?}", cols[index], self.field[index]);
        }
    }
}
