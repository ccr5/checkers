mod player;

use crate::player::player::{create_player, Player};

fn main() {
    println!("Player 1");
    let player_one: Player = create_player();

    println!("Player 2");
    let player_two: Player = create_player();
}
