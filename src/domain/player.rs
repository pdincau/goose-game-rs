#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player {
    name: &'static str,
}

impl Player {
    pub fn new(name: &'static str) -> Player {
        Player { name }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}
