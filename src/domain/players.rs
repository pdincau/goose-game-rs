use std::fmt;

use crate::domain::player::Player;

pub struct Players {
    elements: Vec<Player>,
}

impl Players {
    pub fn find(&self, player: Player) -> Option<&Player> {
        self.elements.iter().find(|&&p| p == player)
    }

    pub fn add(&mut self, player: Player) {
        self.elements.push(player);
    }

    #[allow(dead_code)]
    pub fn first(&self) -> Option<&Player> {
        self.elements.first()
    }

    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.elements.len()
    }
}

impl Default for Players {
    fn default() -> Self {
        Players {
            elements: Vec::new(),
        }
    }
}

impl fmt::Debug for Players {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let player_names: Vec<String> =
            self.elements.iter().map(|p| p.name().to_string()).collect();
        write!(f, "Players: {}", player_names.join(", "))
    }
}
