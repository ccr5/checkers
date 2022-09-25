pub mod field {
    use crate::player::player::Player;

    pub fn create_field(player_one: Player, player_two: Player) -> [[String; 8]; 8] {
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

    pub fn show_field(field: [[String; 8]; 8]) {
        for row in field {
            println!("{:?}", row);
        }
    }
}
