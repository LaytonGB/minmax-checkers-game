use text_io::try_read;

use crate::{
    board::Board, bot::Bot, bot_choice::BotChoice, history::History, io, minmax::MinMax,
    piece::Piece, player::Player, r#move::Move,
};

#[derive(Default, Debug)]
pub struct Checkers {
    board: Board,
    current_player: Player,
    bot_player: Option<(Player, Box<dyn Bot>)>,
    selected_piece: Option<usize>,
    valid_moves: Vec<Move>,
    selectable_positions: Vec<usize>,
    history: History,
}

impl Clone for Checkers {
    fn clone(&self) -> Self {
        Self {
            board: self.board.clone(),
            current_player: self.current_player.clone(),
            bot_player: None,
            selected_piece: self.selected_piece.clone(),
            valid_moves: self.valid_moves.clone(),
            selectable_positions: self.selectable_positions.clone(),
            history: self.history.clone(),
        }
    }
}

impl Checkers {
    pub fn new(bot_player: Option<(Player, BotChoice)>) -> Self {
        let bot_player = if let Some((player, bot_choice)) = bot_player {
            let bot: Box<dyn Bot> = match bot_choice {
                BotChoice::MinMax => Box::new(MinMax),
            };
            Some((player, bot))
        } else {
            None
        };
        Self {
            bot_player,
            ..Default::default()
        }
    }

    pub fn custom_board(board_size: usize, bot_player: Option<(Player, BotChoice)>) -> Self {
        let bot_player = if let Some((player, bot_choice)) = bot_player {
            let bot: Box<dyn Bot> = match bot_choice {
                BotChoice::MinMax => Box::new(MinMax),
            };
            Some((player, bot))
        } else {
            None
        };
        Self {
            board: Board::new(board_size),
            bot_player,
            ..Default::default()
        }
    }

    #[cfg(feature = "standalone")]
    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn start(&mut self) {
        self.update_valid_moves();
        loop {
            #[cfg(feature = "standalone")]
            self.show_board();

            if self
                .bot_player
                .as_ref()
                .and_then(|p| Some(p.0 == self.current_player))
                .unwrap_or(false)
            {
                let bot_move = self
                    .bot_player
                    .as_ref()
                    .unwrap()
                    .1
                    .get_next_move_with_display(&self, None);
                self.make_a_move_from_api(bot_move);
            } else {
                while self.can_move() {
                    self.make_a_move_from_terminal();
                }
            }

            self.end_turn();
            self.announce_new_turn();
            if !self.can_move() {
                #[cfg(feature = "standalone")]
                self.announce_winner(Some(self.current_player.other()));
                break;
            }
        }
    }

    #[cfg(feature = "standalone")]
    fn announce_winner(&mut self, winner: Option<Player>) {
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
    }

    #[cfg(feature = "standalone")]
    fn announce_new_turn(&self) {
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
        !(self.selectable_positions).is_empty()
    }

    fn get_valid_moves(&self) -> Vec<Move> {
        let is_first_move_this_turn = self
            .history
            .get_last_player()
            .unwrap_or(self.current_player.other())
            != self.current_player;
        if !is_first_move_this_turn && !self.history.last_move_was_capture() {
            return Vec::new();
        }

        let all_moves = self.all_moves_for_player(self.current_player);
        let must_cap = self.selected_piece.is_some()
            && (is_first_move_this_turn || self.history.last_move_was_capture())
            || all_moves.iter().any(|m| m.is_capture());
        if must_cap {
            all_moves.into_iter().filter(|m| m.is_capture()).collect()
        } else {
            all_moves
        }
    }

    pub fn get_player_piece_count(&self, player: Player) -> usize {
        self.board
            .get_player_piece_positions(player)
            .fold(0, |x, _| x + 1)
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
                                    let starts_as_king = piece.is_king();
                                    let ends_as_king = starts_as_king
                                        || end_coord.0 == Self::end_row_for_piece(&piece)
                                        || cap_piece.is_king();
                                    return Some(Move::new_capture(
                                        position,
                                        end_pos,
                                        starts_as_king,
                                        ends_as_king,
                                        cap_pos,
                                        cap_piece,
                                    ));
                                }
                            }
                        }
                        return None;
                    } else {
                        let starts_as_king = piece.is_king();
                        let ends_as_king =
                            starts_as_king || cap_coord.0 == Self::end_row_for_piece(&piece);
                        return Some(Move::new_move(
                            position,
                            cap_pos,
                            starts_as_king,
                            ends_as_king,
                        ));
                    }
                }
                None
            })
            .collect()
    }

    pub fn make_a_move_from_api(&mut self, pos: usize) {
        let (row, col) = self.board.to_coord(pos);
        if let Some(position) = self.selected_piece {
            self.move_piece(position, row, col);
            if self.selectable_positions.is_empty() {
                self.end_turn();
            }
        } else {
            self.select_piece(row, col);
        }
    }

    #[cfg(feature = "standalone")]
    fn make_a_move_from_terminal(&mut self) {
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
                    if self.move_piece(position, row, col) {
                        break;
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
                let input = io::get_n_parts(2);
                if input.len() >= 2 {
                    let row: Result<usize, _> = try_read!("{}", input[0].bytes());
                    let col: Result<usize, _> = try_read!("{}", input[1].bytes());
                    if let (Ok(row), Ok(col)) = (row, col) {
                        if self.select_piece(row, col) {
                            break;
                        }
                    } else if input[0] == "undo" {
                        self.undo_last_turn();
                        continue;
                    }
                    println!("ERROR: I didn't catch that, please input your zero-indexed coordinates in format \"ROW <space> COLUMN\".");
                } else if !input.is_empty() && input[0] == "undo" {
                    self.undo_last_turn();
                    break;
                }
            }
        }
    }

    fn move_piece(&mut self, position: usize, row: usize, col: usize) -> bool {
        let started_as_king = self
            .board
            .get(position)
            .expect("piece must exist")
            .is_king();
        let start_coord = self.board.to_coord(position);
        let end_coord = (row, col);
        let end_pos = self.board.to_position(end_coord);
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
                    println!("\nCAPTURED KING PIECE {:?}", self.board.to_coord(cap_pos));
                } else {
                    println!("\nCAPTURED PIECE {:?}", self.board.to_coord(cap_pos));
                }

                self.king_if_end_row(end_pos);
                self.history.push(
                    self.current_player,
                    Move::new_capture(
                        position,
                        end_pos,
                        started_as_king,
                        self.board.get(end_pos).expect("just moved here").is_king(),
                        cap_pos,
                        cap_piece,
                    ),
                );
            } else {
                self.king_if_end_row(end_pos);
                self.history.push(
                    self.current_player,
                    Move::new_move(
                        position,
                        end_pos,
                        started_as_king,
                        self.board.get(end_pos).expect("just moved here").is_king(),
                    ),
                );
            }
            println!(
                "PIECE MOVED {:?} -> {:?}\n",
                start_coord,
                self.board.to_coord(end_pos)
            );
            self.selected_piece = Some(end_pos); // NOTE selected piece update enables chaining captures
            self.update_valid_moves();
            true
        } else {
            println!("ERROR: Invalid position {:?}, please try again.", end_coord);
            false
        }
    }

    fn select_piece(&mut self, row: usize, col: usize) -> bool {
        let coord = (row, col);
        let pos = self.board.to_position(coord);
        if self.selectable_positions.contains(&pos) {
            self.selected_piece = Some(pos);
            println!("\nPIECE SELECTED {:?}\n", coord);
            self.update_selectable_positions();
            let piece = self.board.get(pos).expect("piece must exist");
            let is_king = piece.is_king();
            self.history
                .push(self.current_player, Move::new_select(pos, is_king));
            true
        } else {
            println!("ERROR: Invalid position {:?}, please try again.", coord);
            false
        }
    }

    fn king_if_end_row(&mut self, position: usize) {
        let (pos_row, _) = self.board.to_coord(position);
        if let Some(piece) = self.board.get_mut(position) {
            if !piece.is_king() {
                let end_row = Self::end_row_for_piece(piece);
                if pos_row == end_row {
                    piece.to_king();
                }
            }
        }
    }

    fn end_row_for_piece(piece: &Piece) -> usize {
        match piece.player() {
            Player::Red => 7,
            Player::White => 0,
        }
    }

    pub fn undo_last_turn(&mut self) {
        let last_turn_moves = self.history.pop_last_turn();
        if let Some(mut moves) = last_turn_moves {
            let started_last_turn_as_king = self.history.started_last_turn_as_king();
            while let Some(m) = moves.pop() {
                let mut piece = self.board.take(m.end()).expect("ended turn here");
                if !started_last_turn_as_king {
                    piece.remove_king();
                }
                if let Some((cap_pos, cap_piece)) = m.capture() {
                    self.board.set(cap_pos, Some(cap_piece));
                }
                self.board.set(m.start(), Some(piece));
            }
            self.selectable_positions = Vec::new();
            #[cfg(feature = "standalone")]
            println!("\nLAST TURN UNDONE");
        }
    }

    pub fn undo_last_move(&mut self) {
        if self.selected_piece.is_none() {
            self.selected_piece = Some(self.history.get_last_move().unwrap().end());
        }
        let started_last_move_as_king = self.history.started_last_move_as_king();
        let last_move = self.history.pop_last_move();
        if let Some(m) = last_move {
            let mut piece = self.board.take(m.end()).expect("ended turn here");
            if !started_last_move_as_king {
                piece.remove_king();
            }
            if let Some((cap_pos, cap_piece)) = m.capture() {
                self.board.set(cap_pos, Some(cap_piece));
            }
            self.board.set(m.start(), Some(piece));
            self.selected_piece = Some(m.start());
            self.update_valid_moves();
            #[cfg(feature = "standalone")]
            println!("\nLAST TURN UNDONE");
        } else {
            panic!("no last move to undo")
        }
    }

    pub fn selectable_positions(&self) -> &[usize] {
        self.selectable_positions.as_ref()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn board(&self) -> &Board {
        &self.board
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
            &vec![
                Move::new_move(8, 12, false, false),
                Move::new_move(8, 13, false, false)
            ][..]
        ));
    }
}
