struct Game {
    players: Vec<Player>
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: Vec::new()
        }
    }
    pub fn add(&mut self, player: Player) -> Result<(), GameError> {
        self.players.push(player);
        Ok(())
    }

    pub fn players(&self) -> Vec<Player> {
        let mut vec = Vec::new();
        self.players.iter().for_each(|player| vec.push(player.clone()));

        vec
    }
}

pub enum GameError {
    GenericError
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Player {
    name: &'static str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_a_player() {
        let mut game = Game::new();

        let player = Player { name: "Piero" };

        let result = game.add(player);

        assert!(result.is_ok());

        let players: Vec<Player> = game.players();

        assert_eq!(1, players.len());

        let expected_player = Player { name: "Piero" };

        match players.first() {
            None => panic!("failed!"),
            Some(actual_player) => {
                assert_eq!(expected_player, actual_player.clone())
            }
        }

    }
}
