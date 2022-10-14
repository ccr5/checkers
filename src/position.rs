use std::io;

#[derive(Debug)]
pub struct Position {
    pub column: usize,
    pub row: String,
}

impl Position {
    pub fn new() -> Self {
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

        position
    }

    pub fn convert_row_index(&self) -> usize {
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

        cols.iter().position(|x| x == &self.row).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_row_index() {
        let position: Position = Position {
            column: 2,
            row: "C".to_string(),
        };

        let position_row_number: usize = position.convert_row_index();
        assert_eq!(2, position_row_number);
    }
}
