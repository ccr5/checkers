use crate::player::Player;

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub field: [[i8; 8]; 8],
}

impl Field {
    pub fn new(player_one: &Player, player_two: &Player) -> Self {
        let p1 = player_one.piece_type;
        let p2 = player_two.piece_type;

        let field: [[i8; 8]; 8] = [
            [-1, p1, -1, p1, -1, p1, -1, p1],
            [p1, -1, p1, -1, p1, -1, p1, -1],
            [-1, p1, -1, p1, -1, p1, -1, p1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [p2, -1, p2, -1, p2, -1, p2, -1],
            [-1, p2, -1, p2, -1, p2, -1, p2],
            [p2, -1, p2, -1, p2, -1, p2, -1],
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
        println!("      1 2 3 4 5 6 7 8");
        for index in 0..self.field.len() {
            println!("{} - {:?}", cols[index], self.field[index]);
        }
    }

    pub fn update(mut self, position_row_number: usize, position_column_number: usize, value: i8) {
        self.field[position_row_number][position_column_number - 1] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_field() {
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

        let field_value: [[i8; 8]; 8] = [
            [-1, 1, -1, 1, -1, 1, -1, 1],
            [1, -1, 1, -1, 1, -1, 1, -1],
            [-1, 1, -1, 1, -1, 1, -1, 1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [0, -1, 0, -1, 0, -1, 0, -1],
            [-1, 0, -1, 0, -1, 0, -1, 0],
            [0, -1, 0, -1, 0, -1, 0, -1],
        ];

        let field: Field = Field::new(&player_one, &player_two);
        assert_eq!(field_value, field.field);
    }

    #[test]
    fn test_update() {
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

        let field_value: [[i8; 8]; 8] = [
            [-1, 1, -1, 1, -1, 1, -1, 1],
            [0, -1, 1, -1, 1, -1, 1, -1],
            [-1, 1, -1, 1, -1, 1, -1, 1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [0, -1, 0, -1, 0, -1, 0, -1],
            [-1, 0, -1, 0, -1, 0, -1, 0],
            [0, -1, 0, -1, 0, -1, 0, -1],
        ];

        let field: Field = Field::new(&player_one, &player_two);
        field.update(2, 1, 0);
        assert_eq!(field_value, field.field);
    }
}
