use std::fmt::Display;

use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Piece {
    pub is_king: bool,
    pub player: Player,
}

impl Piece {
    pub fn new(player: Player) -> Self {
        Piece {
            is_king: false,
            player,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match (self.player, self.is_king) {
                (Player::Red, true) => "RK",
                (Player::Red, false) => "R",
                (Player::White, true) => "WK",
                (Player::White, false) => "W",
            }
        )
    }
}
