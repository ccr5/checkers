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

    pub fn update(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field() {
        let player_one: Player = Player {
            name: "Matheus".to_string(),
            piece_type: "White".to_string(),
            matchs: 0,
            victories: 0,
        };

        let player_two: Player = Player {
            name: "Lucas".to_string(),
            piece_type: "Black".to_string(),
            matchs: 0,
            victories: 0,
        };

        let field_value: [[String; 8]; 8] = [
            [
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
            ],
            [
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
            ],
            [
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
                ".....".to_string(),
                "White".to_string(),
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
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
            ],
            [
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
            ],
            [
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
                "Black".to_string(),
                ".....".to_string(),
            ],
        ];

        let field: Field = Field::new(&player_one, &player_two);
        assert_eq!(field_value, field.field);
    }
}
