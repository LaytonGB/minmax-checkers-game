use strum::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Display, Debug)]
pub enum Player {
    #[default]
    Red,
    White,
}

impl Player {
    pub fn other(self) -> Player {
        match self {
            Player::Red => Player::White,
            Player::White => Player::Red,
        }
    }
}
