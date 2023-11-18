use text_io::try_read;

use crate::{board::Board, history::History, player::Player, r#move::Move};

#[derive(Default, Debug)]
pub struct Checkers {
    board: Board,
    current_player: Player,
    bot_player: Option<Player>,
    selected_piece: Option<usize>,
    valid_moves: Vec<Move>,
    selectable_positions: Vec<usize>,
    history: History,
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

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn start(&mut self) {
        self.update_valid_moves();
        loop {
            self.show_board();

            if self
                .bot_player
                .and_then(|p| Some(p == self.current_player))
                .unwrap_or(false)
            {
                todo!("bot turn");
            } else {
                while self.can_move() {
                    self.make_a_move();
                }
            }

            self.end_turn();
            if !self.can_move() {
                self.end_game(Some(self.current_player.other()));
                break;
            }
        }
    }

    fn end_game(&mut self, winner: Option<Player>) {
        if let Some(winner) = winner {
            println!("!!! {} PLAYER WINS !!!", winner);
        } else {
            println!("Game over, nobody wins.");
        }
    }

    fn end_turn(&mut self) {
        self.selected_piece = None;
        self.current_player = self.current_player.other();
        self.update_valid_moves();
        println!("TURN CHANGE\n{} TURN:", self.current_player);
    }

    fn update_valid_moves(&mut self) {
        self.valid_moves = self.get_valid_moves();
        self.update_selectable_positions();
    }

    fn update_selectable_positions(&mut self) {
        self.selectable_positions = if let Some(pos) = self.selected_piece {
            self.valid_moves
                .iter()
                .filter_map(|m| {
                    if m.start() == pos {
                        Some(m.end())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            self.valid_moves.iter().map(|m| m.start()).collect()
        };
        self.selectable_positions.dedup();
    }

    fn can_move(&self) -> bool {
        !self.selectable_positions.is_empty()
    }

    fn get_valid_moves(&self) -> Vec<Move> {
        let all_moves = self.all_moves_for_player(self.current_player);
        let must_cap = self.selected_piece.is_some()
            && self
                .history
                .get_last_player()
                .unwrap_or(self.current_player.other())
                == self.current_player
            && self.history.last_move_was_capture()
            || all_moves.iter().any(|m| m.is_capture());
        if must_cap {
            all_moves.into_iter().filter(|m| m.is_capture()).collect()
        } else {
            all_moves
        }
    }

    fn all_moves_for_player(&self, player: Player) -> Vec<Move> {
        let piece_positions = self.board.get_player_piece_positions(player);
        piece_positions
            .into_iter()
            .flat_map(|pos| self.moves_for_pos(pos))
            .collect()
    }

    fn moves_for_pos(&self, position: usize) -> Vec<Move> {
        let coord = self.board.to_coord(position);
        let piece = self
            .board
            .get(position)
            .expect("piece guaranteed to be present");
        piece
            .directions()
            .iter()
            .filter_map(|d| {
                let (i, j) = *d;
                let cap_coord = (coord.0.wrapping_add(i), coord.1.wrapping_add(j));
                if self.board.is_within_bounds(cap_coord) {
                    let cap_pos = self.board.to_position(cap_coord);
                    if let Some(cap_piece) = self.board.get(cap_pos) {
                        if piece.player() != cap_piece.player() {
                            let end_coord =
                                (cap_coord.0.wrapping_add(i), cap_coord.1.wrapping_add(j));
                            if self.board.is_within_bounds(end_coord) {
                                let end_pos = self.board.to_position(end_coord);
                                if self.board.get(end_pos).is_none() {
                                    Some(Move::new_capture(position, end_pos, cap_pos, cap_piece))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        Some(Move::new(position, cap_pos))
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    /// The player move function. Includes a player selecting a piece, moving a piece, or capturing a piece.
    ///
    /// After any moving or capturing turn this function adds to [`self.history`].
    ///
    /// If the available valid moves are changed, this method updates [`self.valid_moves`].
    fn make_a_move(&mut self) {
        if let Some(position) = self.selected_piece {
            loop {
                println!(
                    "\nPiece at {:?} is selected.\n\
                    Select a position to move it to (row <space> column):\n\
                    Valid positions: {:?}",
                    self.board.to_coord(position),
                    self.selectable_positions
                        .iter()
                        .map(|p| self.board.to_coord(*p))
                        .collect::<Vec<(usize, usize)>>()
                );
                let row: Result<usize, _> = try_read!();
                let col: Result<usize, _> = try_read!();
                if let (Ok(row), Ok(col)) = (row, col) {
                    let coord = (row, col);
                    let end_pos = self.board.to_position(coord);
                    if let Some(m) = self
                        .valid_moves
                        .iter()
                        .find(|m| m.start() == position && m.end() == end_pos)
                    {
                        self.board.r#move(position, end_pos);
                        if let Some((cap_pos, cap_piece)) = m.capture() {
                            self.board.take(cap_pos);
                            if cap_piece.is_king() {
                                self.board.get_mut(end_pos).map(|p| p.to_king());
                                println!(
                                    "\nCAPTURED KING PIECE {:?}",
                                    self.board.to_coord(cap_pos)
                                );
                            } else {
                                println!("\nCAPTURED PIECE {:?}", self.board.to_coord(cap_pos));
                            }

                            self.history.push(
                                self.current_player,
                                Move::new_capture(position, end_pos, cap_pos, cap_piece),
                            );
                        } else {
                            self.history
                                .push(self.current_player, Move::new(position, end_pos));
                        }
                        println!(
                            "PIECE MOVED {:?} -> {:?}\n",
                            coord,
                            self.board.to_coord(end_pos)
                        );
                        self.update_valid_moves();
                        break;
                    } else {
                        println!("ERROR: Invalid position {:?}, please try again.", coord);
                    }
                } else {
                    println!("ERROR: I didn't catch that, please input your zero-indexed coordinates in format \"ROW <space> COLUMN\".");
                }
            }
        } else {
            loop {
                println!(
                    "\nSelect a piece (row <space> column):\n\
                    Movable pieces: {:?}",
                    self.selectable_positions
                        .iter()
                        .map(|p| self.board.to_coord(*p))
                        .collect::<Vec<(usize, usize)>>()
                );
                let row: Result<usize, _> = try_read!();
                let col: Result<usize, _> = try_read!();
                if let (Ok(row), Ok(col)) = (row, col) {
                    let coord = (row, col);
                    let pos = self.board.to_position(coord);
                    if self.selectable_positions.contains(&pos) {
                        self.selected_piece = Some(pos);
                        println!("\nPIECE SELECTED\n");
                        self.update_selectable_positions();
                        break;
                    } else {
                        println!("ERROR: Invalid position {:?}, please try again.", coord);
                    }
                } else {
                    println!("ERROR: I didn't catch that, please input your zero-indexed coordinates in format \"ROW <space> COLUMN\".");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn eq_ignore_order(a: &[Move], b: &[Move]) -> bool {
        let a: HashSet<_> = a.iter().collect();
        let b: HashSet<_> = b.iter().collect();
        a == b
    }

    #[test]
    fn test_moves_for_pos() {
        let checkers = Checkers::default();

        assert!(eq_ignore_order(&checkers.moves_for_pos(0)[..], &vec![][..]));
        assert!(eq_ignore_order(
            &checkers.moves_for_pos(8)[..],
            &vec![Move::new(8, 12), Move::new(8, 13)][..]
        ));
    }

    #[test]
    fn test_make_a_move() {
        todo!("TEST MAKE A MOVE");
    }
}
