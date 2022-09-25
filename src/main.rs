use std::io;

#[derive(Debug)]
struct Player {
    name: String,
    age: i8,
}

fn create_player() -> Player {
    let mut name: String = String::new();
    println!("Digite um nome: ");
    io::stdin().read_line(&mut name).expect("Erro");

    let mut age: String = String::new();
    println!("Digite sua idade: ");
    io::stdin().read_line(&mut age).expect("Deu ruim");

    name.pop();
    age.pop();
    let num_age: i8 = age.parse().expect("Não foi possível converter");

    let player: Player = Player {
        name: name,
        age: num_age,
    };

    return player;
}

fn main() {
    let player_one: Player = create_player();
    let player_two: Player = create_player();

    println!("{:?}", player_one);
    println!("{:?}", player_two);

    // let mut name_two: String = String::new();
    // println!("Digite um nome: ");
    // io::stdin().read_line(&mut name_two).expect("Erro");

    // let mut age_two: String = String::new();
    // println!("Digite sua idade: ");
    // io::stdin().read_line(&mut age_two).expect("Deu ruim");

    // let player_two: Player = Player {
    //     name: name_two,
    //     age: age_two.parse().expect("Não foi possível corrigir"),
    // };
}
