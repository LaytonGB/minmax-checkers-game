use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
}
