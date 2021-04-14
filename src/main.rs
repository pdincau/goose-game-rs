use crate::domain::game::{Game, Player};

mod domain;

fn main() {
    let mut game = Game::new();

    match game.add(Player::new("Pietro")) {
        Ok(_) => println!("{:?}", game.players()),
        Err(error) => println!("{}", error),
    };

    match game.add(Player::new("Pietro")) {
        Ok(_) => println!("{:?}", game.players()),
        Err(error) => println!("{}", error),
    };

    if game.add(Player::new("Paolo")).is_ok() {
        println!("{:?}", game.players());
    }
}

// add integration test on main
// separate command and query? Do we need maybe a new abstraction level?
