use domain::player::Player;

use crate::domain::game::Game;

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

    match game.add(Player::new("Paolo")) {
        Ok(_) => println!("{:?}", game.players()),
        Err(error) => println!("{}", error),
    };
}

// add integration test on main
// separate command and query? Do we need maybe a new abstraction level?
