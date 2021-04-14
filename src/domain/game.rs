use crate::domain::game_error::GameError;
use crate::domain::game_error::GameError::AlreadyInGame;
use crate::domain::player::Player;
use crate::domain::players::Players;

pub struct Game {
    players: Players,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Players::default(),
        }
    }

    pub fn add(&mut self, player: Player) -> Result<(), GameError> {
        match self.players.find(player) {
            None => {
                self.players.add(player);
                Ok(())
            }
            Some(_) => Err(AlreadyInGame(player.name().to_string()))
        }
    }

    pub fn players(&self) -> &Players {
        &self.players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_a_player() {
        let mut game = Game::new();

        let player = Player::new("Piero");

        let result = game.add(player);

        assert!(result.is_ok());

        let players: &Players = game.players();

        assert_eq!(1, players.count());

        let expected_player = Player::new("Piero");

        match players.first() {
            None => panic!("failed!"),
            Some(actual_player) => {
                assert_eq!(expected_player, actual_player.clone())
            }
        }
    }

    #[test]
    fn cannot_add_twice_same_player() {
        let mut game = Game::new();

        let _ = game.add(Player::new("Piero"));

        let result = game.add(Player::new("Piero"));

        assert!(result.is_err());
    }
}
