use anyhow::{anyhow, Result};

use crate::board::Board;
use crate::board_handler::BoardHandler;
use crate::player::Player;
use crate::state::State;

#[derive(Clone, Debug)]
pub struct Checkers {
    pub current_player: Player,
    pub bot_player: Option<Player>,
    pub board: Board,
    pub state: State,
    pub history: Vec<(Player, Vec<usize>)>,
}

impl Default for Checkers {
    fn default() -> Self {
        Self {
            current_player: Player::Red,
            board: Board::new(8),
            bot_player: None,
            state: State::Selecting,
            history: Vec::new(),
        }
    }
}

impl Checkers {
    pub fn new(board_size: usize, bot_player: Option<Player>) -> Self {
        Checkers {
            current_player: Player::Red,
            bot_player,
            board: Board::new(board_size),
            state: State::Selecting,
            history: Vec::new(),
        }
    }

    pub fn select_piece(&mut self, position: usize) -> Result<usize> {
        if self.state != State::Selecting {
            return Err(anyhow!(
                "a piece can only be selected when in the Selection state",
            ));
        }

        if self.board.0[position]
            .and_then(|p| Some(p.player == self.current_player))
            .unwrap_or(false)
        {
            self.state = State::Moving(position);
            Ok(position)
        } else {
            Err(anyhow!(
                "the given position does not contain a selectable piece"
            ))
        }
    }

    pub fn move_piece(&mut self, new_position: usize) -> Result<usize> {
        let (old_position, valid_moves) = match self.state {
            State::Selecting => panic!("attempting to move piece without piece to move"),
            State::Moving(old_position) => (
                old_position,
                BoardHandler::get_valid_moves(&self.board, old_position),
            ),
            State::Chaining(ref past_positions) => {
                let old_position = *past_positions.last().unwrap();
                (
                    old_position,
                    BoardHandler::get_valid_captures(&self.board, old_position)
                        .unwrap_or(Vec::new()),
                )
            }
        };
        if valid_moves.len() == 0 {
            if let State::Chaining(_) = self.state {
                println!("HERE HERE HERE");
                self.end_turn();
                Err(anyhow!("no valid chaining moves, turn has ended"))
            } else {
                unreachable!("cannot run out of moves here");
            }
        } else if valid_moves.iter().any(|(_, p)| *p == new_position) {
            match BoardHandler::move_piece_to(&mut self.board, old_position, new_position) {
                Ok(_possibly_captured_piece) => {
                    if let State::Moving(_) = self.state {
                        self.state = State::Chaining(vec![old_position, new_position]);
                    } else if let State::Chaining(past_positions) = &mut self.state {
                        past_positions.push(new_position);
                    }
                    Ok(new_position)
                }
                Err(e) => Err(e),
            }
        } else {
            Err(anyhow!(
                "new position is not a valid move for selected piece"
            ))
        }
    }

    pub fn end_turn(&mut self) {
        match (&self).state {
            State::Selecting => todo!("current player loses (cant end turn if can capture)"),
            State::Moving(_) => self.switch_player(),
            State::Chaining(_) => self.switch_player(),
        };
        println!(
            "\nTURN CHANGE\nIt is now the {} player's turn.\n",
            self.current_player
        );
    }

    fn switch_player(&mut self) {
        if let State::Chaining(move_history) = std::mem::take(&mut self.state) {
            self.history.push((self.current_player, move_history));
        } else {
            panic!("cannot switch player without being in chaining state");
        }
        match self.current_player {
            Player::Red => self.current_player = Player::White,
            Player::White => self.current_player = Player::Red,
        };
    }

    pub fn print(&self) {
        println!("{}", self.board);
    }
}
