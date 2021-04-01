use crate::domain::game::{Game, GameError, Player};

mod domain;

fn main() {
    let mut game = Game::new();

    if game.add(Player::new("Pietro")).is_ok() {
        format(game.players());
    };
    if game.add(Player::new("Paolo")).is_ok() {
        format(game.players());
    }
}

fn format(players: Vec<Player>) {
    let player_names: Vec<String> = players.iter().map(|p| p.name().to_string()).collect();
    println!("Players: {}", player_names.join(", "));
}