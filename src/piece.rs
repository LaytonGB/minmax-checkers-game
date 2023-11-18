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

    pub fn directions(&self) -> &[(usize, usize)] {
        match (self.is_king, self.player) {
            (true, _) => &DIRECTIONS[..],
            (false, Player::Red) => &DIRECTIONS[..2],
            (false, Player::White) => &DIRECTIONS[2..],
        }
    }
}
