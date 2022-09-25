pub mod player {
    use std::io;

    #[derive(Debug)]
    pub struct Player {
        name: String,
        age: i8,
    }

    pub fn create_player() -> Player {
        let mut name: String = String::new();
        println!("Digite um nome: ");
        io::stdin().read_line(&mut name).expect("Erro");

        let mut age: String = String::new();
        println!("Digite sua idade: ");
        io::stdin().read_line(&mut age).expect("Deu ruim");

        name.pop();
        age.pop();

        let player: Player = Player {
            name: name,
            age: age.parse().expect("Não foi possível converter"),
        };

        return player;
    }
}
