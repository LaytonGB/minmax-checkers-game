use crate::board::Board;
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct Checkers<const BOARD_SIZE_SQUARED_HALVED: usize> {
    current_player: Player,
    human_player: Player,
    board: Board<BOARD_SIZE_SQUARED_HALVED>,
}

impl<const BOARD_SIZE_SQUARED_HALVED: usize> Checkers<BOARD_SIZE_SQUARED_HALVED> {
    pub fn new(human_player: Player) -> Self {
        Checkers {
            current_player: Player::Red,
            human_player,
            board: Board::new(),
        }
    }

    pub fn switch_players(&mut self) {
        match self.current_player {
            Player::Red => self.current_player = Player::White,
            Player::White => self.current_player = Player::Red,
        }
    }

    pub fn get_valid_moves_for_piece(&self, piece_pos: usize) -> Vec<usize> {
        if piece_pos >= BOARD_SIZE_SQUARED_HALVED {
            panic!(
                "piece index out of bounds ({} >= {})",
                piece_pos, BOARD_SIZE_SQUARED_HALVED
            );
        } else if let Some(piece) = self.board.0[piece_pos] {
            self.board.get_valid_moves(piece_pos, piece.player)
        } else {
            panic!("no piece exists at index ({})", piece_pos);
        }
    }

    pub fn take_turn(&mut self, piece_start: usize, piece_end: usize) -> bool {
        if self
            .get_valid_moves_for_piece(piece_start)
            .contains(&piece_end)
        {
            self.board.move_piece(piece_start, piece_end);
            self.switch_players();
            true
        } else {
            false
        }
    }
}
