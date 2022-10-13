use std::io;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub piece_type: String,
    pub matchs: i8,
    pub victories: i8,
}

impl Player {
    pub fn new() -> Player {
        let mut name: String = String::new();
        println!("Insert your name: ");
        io::stdin()
            .read_line(&mut name)
            .expect("Error: We couldn't what you typed");

        let mut piece_type: String = String::new();
        println!("What is your piece type: ");
        io::stdin()
            .read_line(&mut piece_type)
            .expect("Error: We couldn't what you typed");

        name.pop();
        piece_type.pop();

        Player {
            name: name,
            piece_type: piece_type,
            matchs: 0,
            victories: 0,
        }
    }
}

#[derive(Debug)]
pub struct Field {
    field: [[String; 8]; 8],
}

impl Field {
    pub fn new(player_one: &Player, player_two: &Player) -> Field {
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
}
