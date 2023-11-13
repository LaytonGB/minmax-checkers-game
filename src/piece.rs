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
