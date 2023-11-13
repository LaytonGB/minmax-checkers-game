use std::{fmt::Display, iter::repeat};

use strum::IntoEnumIterator;
use tabled::{builder::Builder, settings::Style, tables::IterTable, Table};

use crate::{diagonal::Diagonal, piece::Piece, player::Player};

#[derive(Clone, PartialEq, Debug)]
pub struct Board<const BOARD_SIZE_SQUARED_HALVED: usize>(
    pub [Option<Piece>; BOARD_SIZE_SQUARED_HALVED],
);

impl<const BOARD_SIZE_SQUARED_HALVED: usize> Board<BOARD_SIZE_SQUARED_HALVED> {
    pub fn new() -> Self {
        if BOARD_SIZE_SQUARED_HALVED < 6 || BOARD_SIZE_SQUARED_HALVED % 2 != 0 {
            panic!("board size must be an even integer that is 6 or greater");
        }
        let half_board_size = ((BOARD_SIZE_SQUARED_HALVED * 2) as f32).sqrt() as usize / 2;
        let mut board = [None; BOARD_SIZE_SQUARED_HALVED];
        for i in 0..half_board_size * 3 {
            board[i] = Some(Piece::new(Player::Red));
        }
        for i in BOARD_SIZE_SQUARED_HALVED - 3 * half_board_size..BOARD_SIZE_SQUARED_HALVED {
            board[i] = Some(Piece::new(Player::White));
        }
        Board(board)
    }

    pub fn get_valid_moves(&self, position: usize, current_player: Player) -> Vec<usize> {
        let adjacent = self.get_adjacent(position);
        let captures =
            self.get_valid_captures(position, current_player, [false; BOARD_SIZE_SQUARED_HALVED]);
        adjacent.into_iter().chain(captures.into_iter()).collect()
    }

    fn get_adjacent(&self, index: usize) -> Vec<usize> {
        Diagonal::iter()
            .filter_map(|d| self.get_next_in_direction(index, d))
            .collect()
    }

    fn get_board_size(&self) -> usize {
        ((BOARD_SIZE_SQUARED_HALVED * 2) as f32).sqrt() as usize
    }

    fn get_next_in_direction(&self, origin: usize, direction: Diagonal) -> Option<usize> {
        let half_board_size = self.get_board_size();
        let adjustment = if (origin / half_board_size) % 2 == 0 {
            0
        } else {
            1
        };
        let res = match direction {
            Diagonal::UpLeft => origin.wrapping_sub(half_board_size + adjustment),
            Diagonal::UpRight => origin.wrapping_sub(half_board_size + adjustment - 1),
            Diagonal::DownLeft => origin + half_board_size - adjustment,
            Diagonal::DownRight => origin + half_board_size - adjustment + 1,
        };
        if res < BOARD_SIZE_SQUARED_HALVED {
            Some(res)
        } else {
            None
        }
    }

    // FIXME must accomodate piece kinging
    fn get_valid_captures(
        &self,
        position: usize,
        current_player: Player,
        visited: [bool; BOARD_SIZE_SQUARED_HALVED],
    ) -> Vec<usize> {
        let mut res = Vec::new();
        for d in Diagonal::iter() {
            match self.get_next_in_direction(position, d) {
                Some(pos)
                    if self.0[pos]
                        .and_then(|p| Some(p.player != current_player))
                        .unwrap_or(false) =>
                {
                    res.extend(
                        &mut self
                            .get_adjacent(pos)
                            .into_iter()
                            .filter(|&p| self.0[p].is_none())
                            .map(|p| {
                                let mut v = vec![p];
                                let mut vis = visited.clone();
                                vis[position] = true;
                                v.append(&mut self.get_valid_captures(p, current_player, vis));
                                v
                            }),
                    )
                }
                _ => (),
            };
        }
        vec![]
    }

    // pub fn find_path_and_captured_pieces(
    //     &self,
    //     start_pos: usize,
    //     end_pos: usize,
    // ) -> (Vec<usize>, Vec<usize>) {
    // }

    pub fn move_piece(&mut self, start_pos: usize, end_pos: usize) {
        todo!("piece must be moved and also take max number of pieces during move")
    }
}

impl<const BOARD_SIZE_SQUARED_HALVED: usize> Display for Board<BOARD_SIZE_SQUARED_HALVED> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_size = self.get_board_size();
        let mut builder = Builder::default();
        let mut data: Vec<&str> = self
            .0
            .iter()
            .zip(repeat("⬜"))
            .flat_map(|(p, sq)| {
                [
                    sq,
                    match p {
                        Some(p) => match (p.player, p.is_king) {
                            (Player::Red, true) => "RK",
                            (Player::Red, false) => "R",
                            (Player::White, true) => "WK",
                            (Player::White, false) => "W",
                        },
                        None => "⬛",
                    },
                ]
            })
            .collect();
        let mut data: Vec<Vec<&str>> = data.chunks(board_size).map(move |r| Vec::from(r)).collect();
        for i in (1..board_size).step_by(2) {
            let s = data[i].remove(0);
            data[i].push(s);
        }
        for row in data.into_iter() {
            builder.push_record(row);
        }
        let table = builder
            .build()
            .with(Style::ascii().remove_horizontals())
            .to_string();
        write!(f, "{}", table)
    }
}
