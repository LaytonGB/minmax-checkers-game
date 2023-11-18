use std::fmt::Display;

use crate::{constants::DIRECTIONS, player::Player};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Piece {
    is_king: bool,
    player: Player,
}

impl Piece {
    pub fn red() -> Piece {
        Self {
            is_king: false,
            player: Player::Red,
        }
    }

    pub fn white() -> Piece {
        Self {
            is_king: false,
            player: Player::White,
        }
    }

    pub fn is_king(&self) -> bool {
        self.is_king
    }

    pub fn player(&self) -> Player {
        self.player
    }

    pub fn to_king(&mut self) {
        self.is_king = true;
    }

    pub fn directions(&self) -> &[(usize, usize)] {
        match (self.is_king, self.player) {
            (true, _) => &DIRECTIONS[..],
            (false, Player::Red) => &DIRECTIONS[..2],
            (false, Player::White) => &DIRECTIONS[2..],
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.player, self.is_king) {
                (Player::Red, true) => "😡",
                (Player::Red, false) => "🔴",
                (Player::White, true) => "🐻‍❄️",
                (Player::White, false) => "⚪",
            }
            .to_owned()
        )
    }
}
