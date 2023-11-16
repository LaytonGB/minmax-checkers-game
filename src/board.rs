use std::{fmt::Display, iter::repeat};

use tabled::{builder::Builder, settings::Style};

use crate::{diagonal::Diagonal, piece::Piece, player::Player};

#[derive(Clone, PartialEq, Debug)]
pub struct Board(pub Vec<Option<Piece>>);

impl Board {
    pub fn new(board_size: usize) -> Self {
        if board_size < 6 || board_size % 2 != 0 {
            panic!("board size must be an even integer that is 6 or greater");
        }

        let board_size_squared_halved = board_size.pow(2) / 2;
        let half_board_size = board_size / 2;
        let mut board = vec![None; board_size_squared_halved];
        for i in 0..half_board_size * 3 {
            board[i] = Some(Piece::new(Player::Red));
        }
        for i in board_size_squared_halved - 3 * half_board_size..board_size_squared_halved {
            board[i] = Some(Piece::new(Player::White));
        }
        Board(board)
    }

    pub fn get_board_size(&self) -> usize {
        ((self.0.len() * 2) as f32).sqrt() as usize
    }

    pub fn to_coord(&self, position: usize) -> (usize, usize) {
        let board_size = self.get_board_size();
        let row = position * 2 / board_size;
        let adjustment = if row % 2 == 0 { 1 } else { 0 };
        (row, (position % (board_size / 2)) * 2 + adjustment)
    }

    pub fn to_position(&self, coord: (usize, usize)) -> usize {
        let board_size = self.get_board_size();
        let (y, x) = coord;
        y * board_size / 2 + x / 2
    }

    pub fn get_next_in_direction(&self, origin: usize, direction: Diagonal) -> Option<usize> {
        let board_size = self.get_board_size();
        let adjustment = if (origin / board_size) % 2 == 0 { 0 } else { 1 };
        let res = match direction {
            Diagonal::UpLeft => origin.wrapping_sub(board_size + adjustment),
            Diagonal::UpRight => origin.wrapping_sub(board_size + adjustment - 1),
            Diagonal::DownLeft => origin + board_size - adjustment,
            Diagonal::DownRight => origin + board_size - adjustment + 1,
        };
        if res < self.0.len() {
            Some(res)
        } else {
            None
        }
    }

    pub fn get(&self, coord: (usize, usize)) -> Option<Piece> {
        self.0[self.to_position(coord)]
    }

    pub fn set(&mut self, coord: (usize, usize), new_value: Option<Piece>) -> Option<Piece> {
        let pos = self.to_position(coord);
        let mut res = new_value;
        std::mem::swap(&mut self.0[pos], &mut res);
        res
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_size = self.get_board_size();
        let mut builder = Builder::default();
        let data: Vec<&str> = self
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
        let table = builder.build().with(Style::dots()).to_string();
        write!(f, "{}", table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_coord() {
        /*
        __  0 __  1 __  2 __  3 || ___ 0,1 ___ 0,3 ___ 0,5 ___ 0,7
        4  __  5 __  6 __  7 __ || 1,0 ___ 1,2 ___ 1,4 ___ 1,6 ___
        __  8 __  9 __ 10 __ 11 || ___ 2,1 ___ 2,3 ___ 2,5 ___ 2,7
        12 __ 13 __ 14 __ 15 __ || 3,0 ___ 3,2 ___ 3,4 ___ 3,6 ___
        __ 16 __ 17 __ 18 __ 19 || ___ 4,1 ___ 4,3 ___ 4,5 ___ 4,7
        20 __ 21 __ 22 __ 23 __ || 5,0 ___ 5,2 ___ 5,4 ___ 5,6 ___
        __ 24 __ 25 __ 26 __ 27 || ___ 6,1 ___ 6,3 ___ 6,5 ___ 6,7
        28 __ 29 __ 30 __ 31 __ || 7,0 ___ 7,2 ___ 7,4 ___ 7,6 ___
        */

        let board: Board = Board::new(8);

        assert_eq!(board.to_coord(0), (0, 1));
        assert_eq!(board.to_coord(1), (0, 3));
        assert_eq!(board.to_coord(4), (1, 0));
        assert_eq!(board.to_coord(3), (0, 7));
        assert_eq!(board.to_coord(31), (7, 6));
    }

    #[test]
    fn test_to_position() {
        /*
        __  0 __  1 __  2 __  3 || ___ 0,1 ___ 0,3 ___ 0,5 ___ 0,7
        4  __  5 __  6 __  7 __ || 1,0 ___ 1,2 ___ 1,4 ___ 1,6 ___
        __  8 __  9 __ 10 __ 11 || ___ 2,1 ___ 2,3 ___ 2,5 ___ 2,7
        12 __ 13 __ 14 __ 15 __ || 3,0 ___ 3,2 ___ 3,4 ___ 3,6 ___
        __ 16 __ 17 __ 18 __ 19 || ___ 4,1 ___ 4,3 ___ 4,5 ___ 4,7
        20 __ 21 __ 22 __ 23 __ || 5,0 ___ 5,2 ___ 5,4 ___ 5,6 ___
        __ 24 __ 25 __ 26 __ 27 || ___ 6,1 ___ 6,3 ___ 6,5 ___ 6,7
        28 __ 29 __ 30 __ 31 __ || 7,0 ___ 7,2 ___ 7,4 ___ 7,6 ___
        */

        let board: Board = Board::new(8);

        assert_eq!(board.to_position((0, 1)), 0);
        assert_eq!(board.to_position((0, 3)), 1);
        assert_eq!(board.to_position((1, 0)), 4);
        assert_eq!(board.to_position((0, 7)), 3);
        assert_eq!(board.to_position((7, 6)), 31);
    }
}
