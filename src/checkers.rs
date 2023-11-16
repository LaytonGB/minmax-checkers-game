use crate::board::Board;
use crate::board_handler::BoardHandler;
use crate::player::Player;
use crate::state::State;

#[derive(Clone, Debug)]
pub struct Checkers {
    pub current_player: Player,
    pub human_player: Player,
    pub board: Board,
    pub state: State,
}

impl Checkers {
    pub fn new(human_player: Player, board_size: usize) -> Self {
        Checkers {
            current_player: Player::Red,
            human_player,
            board: Board::new(board_size),
            state: State::Selecting,
        }
    }

    pub fn get_valid_moves_for_piece(&self, piece_pos: usize) -> Vec<usize> {
        if let Some(piece) = self.board.0[piece_pos] {
            BoardHandler::get_valid_moves(&self.board, piece_pos, piece.player)
        } else {
            Vec::new()
        }
    }

    pub fn select_piece(&mut self, selected_piece: usize) -> Option<usize> {
        if self.state != State::Selecting {
            panic!("cannot select piece when not in Selecting state");
        }

        if self.board.0[selected_piece]
            .and_then(|p| Some(p.player == self.current_player))
            .unwrap_or(false)
        {
            self.state = State::Moving;
            Some(selected_piece)
        } else {
            None
        }
    }

    pub fn move_piece(&mut self, piece_start: usize, piece_end: usize) -> Option<usize> {
        if self
            .get_valid_moves_for_piece(piece_start)
            .contains(&piece_end)
        {
            BoardHandler::move_piece(&mut self.board, piece_start, piece_end);
            if self.get_valid_moves_for_piece(piece_end).len() == 0 {
                self.end_turn_or_cancel_selection();
            } else if self.state == State::Moving {
                self.state = State::Chaining;
            }
            Some(piece_end)
        } else {
            None
        }
    }

    pub fn end_turn_or_cancel_selection(&mut self) {
        match self.state {
            State::Selecting => todo!("current player loses"),
            State::Moving => todo!("back to selecting state"),
            State::Chaining => self.switch_player(),
        }
    }

    pub fn switch_player(&mut self) {
        match self.current_player {
            Player::Red => self.current_player = Player::White,
            Player::White => self.current_player = Player::Red,
        }
    }

    pub fn print(&self) {
        println!("{}", self.board);
    }
}
