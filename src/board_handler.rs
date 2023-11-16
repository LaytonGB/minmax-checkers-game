use crate::{board::Board, diagonal::Diagonal, player::Player};

use strum::IntoEnumIterator;

pub struct BoardHandler;

impl BoardHandler {
    pub fn get_valid_moves(board: &Board, position: usize, current_player: Player) -> Vec<usize> {
        let adjacent = Self::get_adjacent(board, position);
        let captures =
            Self::get_valid_captures(board, position, current_player, vec![false; board.0.len()]);
        adjacent.into_iter().chain(captures.into_iter()).collect()
    }

    pub fn get_adjacent(board: &Board, index: usize) -> Vec<usize> {
        Diagonal::iter()
            .filter_map(|d| board.get_next_in_direction(index, d))
            .collect()
    }

    // FIXME must accomodate piece kinging
    // FIXME should avoid jumping over the same space twice, NOT avoid jumping back to visited position
    pub fn get_valid_captures(
        board: &Board,
        position: usize,
        current_player: Player,
        visited: Vec<bool>,
    ) -> Vec<usize> {
        let mut res = Vec::new();
        for d in Diagonal::iter() {
            match board.get_next_in_direction(position, d) {
                Some(pos)
                    if board.0[pos]
                        .and_then(|p| Some(p.player != current_player))
                        .unwrap_or(false) =>
                {
                    res.extend(
                        Self::get_adjacent(board, pos)
                            .into_iter()
                            .filter(|&p| board.0[p].is_none())
                            .map(|p| {
                                let mut v = vec![p];
                                let mut vis = visited.clone();
                                vis[position] = true;
                                v.append(&mut Self::get_valid_captures(
                                    board,
                                    p,
                                    current_player,
                                    vis,
                                ));
                                v
                            }),
                    )
                }
                _ => (),
            };
        }
        vec![]
    }

    pub fn move_piece(board: &mut Board, start_pos: usize, end_pos: usize) {
        let (ay, ax) = board.to_coord(start_pos);
        let (by, bx) = board.to_coord(end_pos);
        let (dy, dx) = (ay as i32 - by as i32, ax as i32 - bx as i32);
        let dabs = dy.abs();
        if dabs != dx.abs() {
            panic!("piece was moved non-diagonally");
        }

        match dabs {
            2 => {
                let captured_piece_coord =
                    ((ay as i32 - dy / 2) as usize, (ax as i32 - dx / 2) as usize);
                board.set(captured_piece_coord, None);
            }
            1 => (),
            _ => unreachable!("piece can only move 1 or 2 spaces"),
        }
        board.0[end_pos] = std::mem::take(&mut board.0[start_pos]);
    }

    pub fn count_player_pieces(board: &Board, player: Player) -> usize {
        board
            .0
            .iter()
            .filter(|p| {
                if let Some(ref p) = p {
                    p.player == player
                } else {
                    false
                }
            })
            .count()
    }
}
