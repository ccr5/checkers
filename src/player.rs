use std::io;

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub piece_type: String,
    pub matchs: i8,
    pub victories: i8,
}

impl Player {
    pub fn new() -> Self {
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
