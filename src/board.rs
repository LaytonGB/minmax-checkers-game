use std::{fmt::Display, iter::repeat};

use tabled::tables::IterTable;

use crate::{piece::Piece, player::Player};

/// A struct that contains a board with numbered posiitions matching those shown here.
/// Use the [`to_coord`] and [`to_position`] methods to translate positions from left example
/// to right example or translate coords from right example to left example respectively.
///
/// __  0 __  1 __  2 __  3 || ___ 0,1 ___ 0,3 ___ 0,5 ___ 0,7
/// 4  __  5 __  6 __  7 __ || 1,0 ___ 1,2 ___ 1,4 ___ 1,6 ___
/// __  8 __  9 __ 10 __ 11 || ___ 2,1 ___ 2,3 ___ 2,5 ___ 2,7
/// 12 __ 13 __ 14 __ 15 __ || 3,0 ___ 3,2 ___ 3,4 ___ 3,6 ___
/// __ 16 __ 17 __ 18 __ 19 || ___ 4,1 ___ 4,3 ___ 4,5 ___ 4,7
/// 20 __ 21 __ 22 __ 23 __ || 5,0 ___ 5,2 ___ 5,4 ___ 5,6 ___
/// __ 24 __ 25 __ 26 __ 27 || ___ 6,1 ___ 6,3 ___ 6,5 ___ 6,7
/// 28 __ 29 __ 30 __ 31 __ || 7,0 ___ 7,2 ___ 7,4 ___ 7,6 ___
///
/// # Example
///
/// ```
/// use kar_checkers_minmax::board::Board;
/// let board = Board::new(8);
/// assert_eq!(board.to_coord(16), (4, 1));
/// assert_eq!(board.to_position((4, 5)), 18);
/// ```
#[derive(Debug)]
pub struct Board {
    board: Vec<Option<Piece>>,
    size: usize,
    half_size: usize,
    position_count: usize,
}

impl Board {
    pub fn new(board_size: usize) -> Self {
        Self::validate_board_size(board_size);
        let half_size = board_size / 2;
        let position_count = board_size.pow(2) / 2;
        let board = (0..half_size * 3)
            .map(|_| Some(Piece::red()))
            .chain((0..half_size * (board_size - 6)).map(|_| None))
            .chain((0..half_size * 3).map(|_| Some(Piece::white())))
            .collect();

        Self {
            board,
            size: board_size,
            half_size,
            position_count,
        }
    }

    /// For testing. Creates a Board with a supplied layout.
    ///
    /// # Example
    ///
    /// ```
    /// use kar_checkers_minmax::{board::Board, piece::Piece};
    /// let board = Board::with_layout(8, vec![
    ///     Some(Piece::red()), Some(Piece::red()), Some(Piece::red()), Some(Piece::red()),
    ///     Some(Piece::red()), Some(Piece::red()), Some(Piece::red()), Some(Piece::red()),
    ///     Some(Piece::red()), Some(Piece::red()), Some(Piece::red()), Some(Piece::red()),
    ///     None, None, None, None,
    ///     None, None, None, None,
    ///     Some(Piece::white()), Some(Piece::white()), Some(Piece::white()), Some(Piece::white()),
    ///     Some(Piece::white()), Some(Piece::white()), Some(Piece::white()), Some(Piece::white()),
    ///     Some(Piece::white()), Some(Piece::white()), Some(Piece::white()), Some(Piece::white()),
    /// ]);
    /// assert_eq!(board, Board::new(8));
    /// ```
    pub fn with_layout(board_size: usize, board: Vec<Option<Piece>>) -> Self {
        Self::validate_board_size(board_size);
        Self {
            board,
            size: board_size,
            half_size: board_size / 2,
            position_count: board_size.pow(2) / 2,
        }
    }

    #[inline]
    fn validate_board_size(board_size: usize) {
        if board_size % 2 == 1 || board_size < 6 {
            panic!("invalid board size");
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn half_size(&self) -> usize {
        self.half_size
    }

    #[inline]
    pub fn position_count(&self) -> usize {
        self.position_count
    }

    #[inline]
    pub fn get(&self, position: usize) -> Option<Piece> {
        self.board[position]
    }

    #[inline]
    pub fn get_mut(&mut self, position: usize) -> Option<&mut Piece> {
        self.board[position].as_mut()
    }

    #[inline]
    pub fn set(&mut self, position: usize, new_value: Option<Piece>) {
        self.board[position] = new_value;
    }

    #[inline]
    pub fn r#move(&mut self, start_pos: usize, end_pos: usize) {
        self.board[end_pos] = std::mem::take(&mut self.board[start_pos]);
    }

    #[inline]
    pub fn take(&mut self, position: usize) -> Option<Piece> {
        std::mem::take(&mut self.board[position])
    }

    #[inline]
    pub fn is_within_bounds(&self, coord: (usize, usize)) -> bool {
        let (y, x) = coord;
        y < self.size && x < self.size
    }

    pub fn get_player_piece_positions(&self, player: Player) -> impl Iterator<Item = usize> + '_ {
        self.board.iter().enumerate().filter_map(move |(i, p)| {
            if p.and_then(|p| Some(p.player() == player)).unwrap_or(false) {
                Some(i)
            } else {
                None
            }
        })
    }

    pub fn to_coord(&self, position: usize) -> (usize, usize) {
        let row = position / self.half_size;
        (row, position % self.half_size * 2 + (row + 1) % 2)
    }

    pub fn to_position(&self, coord: (usize, usize)) -> usize {
        let (row, col) = coord;
        row * self.half_size + col / 2
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new(8)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let with_squares = self
            .board
            .iter()
            .zip(repeat("🟫".to_owned()))
            .enumerate()
            .flat_map(|(i, (p, sq))| {
                if (i / self.half_size) % 2 == 0 {
                    vec![
                        sq,
                        p.and_then(|p| Some(format!("{}", p)))
                            .unwrap_or("⬛".to_owned()),
                    ]
                } else {
                    vec![
                        p.and_then(|p| Some(format!("{}", p)))
                            .unwrap_or("⬛".to_owned()),
                        sq,
                    ]
                }
            })
            .collect::<Vec<String>>();
        let rows = with_squares.chunks(self.size).map(|strings| strings);
        write!(f, "{}", IterTable::new(rows).to_string())
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.board == other.board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_coords() {
        let board = Board::new(8);

        assert_eq!(board.to_coord(0), (0, 1));
        assert_eq!(board.to_coord(3), (0, 7));
        assert_eq!(board.to_coord(4), (1, 0));
        assert_eq!(board.to_coord(5), (1, 2));
        assert_eq!(board.to_coord(26), (6, 5));
        assert_eq!(board.to_coord(31), (7, 6));
    }

    #[test]
    fn test_to_position() {
        let board = Board::new(8);

        assert_eq!(board.to_position((0, 1)), 0);
        assert_eq!(board.to_position((0, 7)), 3);
        assert_eq!(board.to_position((1, 0)), 4);
        assert_eq!(board.to_position((1, 2)), 5);
        assert_eq!(board.to_position((6, 5)), 26);
        assert_eq!(board.to_position((7, 6)), 31);
    }
}
