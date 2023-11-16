use strum::Display;

use crate::diagonal::Diagonal;

#[derive(Clone, Copy, PartialEq, Eq, Display, Debug)]
pub enum Player {
    Red,
    White,
}

impl Player {
    pub fn other(self) -> Self {
        match self {
            Player::Red => Player::White,
            Player::White => Player::Red,
        }
    }

    pub fn direction(self) -> (Diagonal, Diagonal) {
        match self {
            Player::Red => (Diagonal::DownLeft, Diagonal::DownRight),
            Player::White => (Diagonal::UpLeft, Diagonal::UpRight),
        }
    }
}
