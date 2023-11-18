use crate::{board::Board, history::Turn, player::Player};

#[derive(Default, Debug)]
pub struct Checkers {
    pub board: Board,
    pub current_player: Player,
    pub bot_player: Option<Player>,
    pub selected_piece: Option<usize>,
    pub turn: Option<Turn>,
    pub history: Vec<(Player, Turn)>,
    pub valid_moves: Vec<(Option<usize>, usize)>,
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

    pub fn start(&mut self) {
        loop {
            if self
                .bot_player
                .and_then(|p| Some(p == self.current_player))
                .unwrap_or(false)
            {
                todo!("bot turn");
            } else {
                while self.can_move() {
                    self.player_turn();
                }
            }

            self.end_turn();
            self.update_valid_moves();
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
        todo!()
    }

    fn update_valid_moves(&mut self) {
        if let Some(start_pos) = self.selected_piece {
            self.valid_moves = self.valid_moves_for_pos(start_pos);
        } else {
            self.valid_moves = self.movable_pieces();
        }
    }

    fn can_move(&self) -> bool {
        !self.valid_moves.is_empty()
    }

    fn movable_pieces(&self) -> Vec<(Option<usize>, usize)> {
        let mut can_cap = false;
        let all_possible_moves: Vec<(Option<usize>, usize)> = self
            .board
            .get_player_piece_positions(self.current_player)
            .flat_map(|start_pos| {
                let start_coord = self.board.to_coord(start_pos);
                let start_piece = self
                    .board
                    .get(start_pos)
                    .expect("piece guaranteed to be present");
                start_piece
                    .directions()
                    .iter()
                    .filter_map(|d| {
                        let (i, j) = *d;
                        let cap_coord = (start_coord.0 + i, start_coord.1 + j);
                        let cap_pos = self.board.to_position(cap_coord);
                        if let Some(cap_piece) = self.board.get(cap_pos) {
                            if start_piece.player() != cap_piece.player() {
                                let end_coord = (cap_coord.0 + i, cap_coord.1 + j);
                                let end_pos = self.board.to_position(end_coord);
                                if self.board.get(end_pos).is_none() {
                                    can_cap = true;
                                    Some((Some(cap_pos), start_pos))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            Some((None, start_pos))
                        }
                    })
                    .collect::<Vec<(Option<usize>, usize)>>()
            })
            .collect();
        if can_cap {
            all_possible_moves
                .iter()
                .filter_map(|p| if p.0.is_some() { Some(*p) } else { None })
                .collect()
        } else {
            all_possible_moves.iter().map(|p| *p).collect()
        }
    }

    fn valid_moves_for_pos(&self, position: usize) -> Vec<(Option<usize>, usize)> {
        let must_cap = if let Some((_, turn)) = self.history.last() {
            match turn {
                Turn::Captures(_) => true,
                Turn::Moves(_) => false,
            }
        } else {
            false
        };
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
                let cap_coord = (coord.0 + i, coord.1 + j);
                let cap_pos = self.board.to_position(cap_coord);
                if let Some(cap_piece) = self.board.get(cap_pos) {
                    if piece.player() != cap_piece.player() {
                        let end_coord = (cap_coord.0 + i, cap_coord.1 + j);
                        let end_pos = self.board.to_position(end_coord);
                        if self.board.get(end_pos).is_none() {
                            Some((Some(cap_pos), position))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else if must_cap {
                    None
                } else {
                    Some((None, position))
                }
            })
            .collect()
    }

    fn player_turn(&mut self) {
        todo!()
    }
}
