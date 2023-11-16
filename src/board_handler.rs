use std::slice;

use crate::{board::Board, diagonal::Diagonal, piece::Piece, player::Player};

use anyhow::{anyhow, Result};
use strum::IntoEnumIterator;

const DIRECTION_UNITS: [(usize, usize); 4] = [
    (1, 1),
    (1, usize::MAX),
    (usize::MAX, usize::MAX),
    (usize::MAX, 1),
];

pub struct BoardHandler;

impl BoardHandler {
    pub fn get_adjacent(board: &Board, index: usize) -> Vec<usize> {
        Diagonal::iter()
            .filter_map(|d| board.get_next_in_direction(index, d))
            .collect()
    }

    pub fn get_valid_moves(board: &Board, position: usize) -> Vec<(Option<Piece>, usize)> {
        let captures = Self::get_valid_captures(board, position);
        if let Ok(captures) = captures {
            if captures.len() > 0 {
                return captures;
            }
        }
        let adjacent = Self::get_valid_adjacents(board, position);
        if let Ok(adjacent) = adjacent {
            if adjacent.len() > 0 {
                return adjacent;
            }
        }
        Vec::new()
    }

    pub fn get_valid_adjacents(
        board: &Board,
        position: usize,
    ) -> Result<Vec<(Option<Piece>, usize)>> {
        if let Some(piece) = board.0[position] {
            let (y, x) = board.to_coord(position);
            let directions = Self::get_direction_iter_for_piece(&piece);
            Ok(directions.fold(Vec::with_capacity(4), |mut v, (i, j)| {
                ((y, x));
                let (row, col) = (y.wrapping_add(*i), x.wrapping_add(*j));
                let board_size = board.get_board_size();
                let half_board_size = board_size / 2;
                if row < board_size && col < half_board_size {
                    let pos = board.to_position((row, col));
                    if let None = board.0[pos] {
                        v.push((None, pos));
                    }
                }
                v
            }))
        } else {
            Err(anyhow!("position does not contain a piece"))
        }
    }

    // FIXME must accomodate piece kinging
    // FIXME should avoid jumping over the same space twice, NOT avoid jumping back to visited position
    pub fn get_valid_captures(
        board: &Board,
        position: usize,
    ) -> Result<Vec<(Option<Piece>, usize)>> {
        if let Some(piece) = board.0[position] {
            let (y, x) = board.to_coord(position);
            let directions = Self::get_direction_iter_for_piece(&piece);
            Ok(directions.fold(Vec::with_capacity(4), |mut v, (i, j)| {
                let (row1, col1) = (y.wrapping_add(*i), x.wrapping_add(*j));
                let half_board_size = board.get_board_size() / 2;
                if row1 < half_board_size && col1 < half_board_size {
                    let pos1 = board.to_position((row1, col1));
                    if let Some(piece_to_be_captured) = board.0[pos1] {
                        let (row2, col2) = (row1.wrapping_add(*i), col1.wrapping_add(*j));
                        if row2 < half_board_size && col2 < half_board_size {
                            let pos2 = board.to_position((row2, col2));
                            if let None = board.0[pos2] {
                                v.push((Some(piece_to_be_captured), pos2));
                            }
                        }
                    }
                }
                v
            }))
        } else {
            Err(anyhow!("position does not contain a piece"))
        }
    }

    fn get_direction_iter_for_piece(piece: &Piece) -> slice::Iter<'_, (usize, usize)> {
        match (piece.is_king, piece.player) {
            (true, _) => DIRECTION_UNITS[..].iter(),
            (false, Player::Red) => DIRECTION_UNITS[..2].iter(),
            (false, Player::White) => DIRECTION_UNITS[2..].iter(),
        }
    }

    pub fn movable_pieces_for_player(board: &Board, player: Player) -> Vec<usize> {
        board
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if p.and_then(|p| {
                    Some(p.player == player && BoardHandler::get_valid_moves(board, i).len() > 0)
                })
                .unwrap_or(false)
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn move_piece_to(
        board: &mut Board,
        start_pos: usize,
        end_pos: usize,
    ) -> Result<Option<Piece>> {
        match Self::is_move_valid(board, start_pos, end_pos) {
            Ok(Some(captured_piece_coords)) => {
                let captured_piece = board.set(captured_piece_coords, None);
                board.0[end_pos] = std::mem::take(&mut board.0[start_pos]);
                Ok(captured_piece)
            }
            Ok(None) => {
                board.0[end_pos] = std::mem::take(&mut board.0[start_pos]);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn is_move_valid(
        board: &Board,
        start_pos: usize,
        end_pos: usize,
    ) -> Result<Option<(usize, usize)>> {
        let (ay, ax) = board.to_coord(start_pos);
        let (by, bx) = board.to_coord(end_pos);
        let (dy, dx) = (ay as i32 - by as i32, ax as i32 - bx as i32);
        let dabs = dy.abs();
        if dabs != dx.abs() {
            return Err(anyhow!(
                "moves must be diagonal ({:?} -> {:?})",
                (ay, ax),
                (by, bx)
            ));
        }

        match dabs {
            2 => {
                let captured_piece_coord =
                    ((ay as i32 + dy / 2) as usize, (ax as i32 + dx / 2) as usize);
                Ok(Some(captured_piece_coord))
            }
            1 => Ok(None),
            moves => Err(anyhow!(
                "piece can only move 1 space, or 2 if capturing (moved {})",
                moves
            )),
        }
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
