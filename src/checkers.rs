use crate::{
    board::Board,
    history::{History, Turn},
    player::Player,
};

#[derive(Default, Debug)]
pub struct Checkers {
    pub board: Board,
    pub current_player: Player,
    pub bot_player: Option<Player>,
    pub selected_piece: Option<usize>,
    pub turn: Option<Turn>,
    pub history: History,
}

impl Checkers {
    pub fn new(bot_player: Option<Player>) -> Self {
        Self {
            bot_player,
            ..Default::default()
        }
    }

    pub fn custom_board(board_size: usize, bot_player: Option<Player>) -> Self {
        Self {
            board: Board::new(board_size),
            bot_player,
            ..Default::default()
        }
    }
}
