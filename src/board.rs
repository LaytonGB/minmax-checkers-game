use crate::piece::Piece;

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
/// assert_eq!(board.to_coords(16), (4, 1));
/// assert_eq!(board.to_position((4, 5)), 18);
/// ```
///
pub struct Board {
    board: Vec<Option<Piece>>,
    size: usize,
    half_size: usize,
    position_count: usize,
}

impl Board {
    pub fn new(board_size: usize) -> Board {
        if board_size % 2 == 1 || board_size < 6 {
            panic!("invalid board size");
        }

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

    pub fn to_coords(&self, position: usize) -> (usize, usize) {
        let row = position / self.half_size;
        (row, position % self.half_size * 2 + (row + 1) % 2)
    }

    pub fn to_position(&self, coord: (usize, usize)) -> usize {
        let (row, col) = coord;
        row * self.half_size + col / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_coords() {
        let board = Board::new(8);

        assert_eq!(board.to_coords(0), (0, 1));
        assert_eq!(board.to_coords(3), (0, 7));
        assert_eq!(board.to_coords(4), (1, 0));
        assert_eq!(board.to_coords(5), (1, 2));
        assert_eq!(board.to_coords(26), (6, 5));
        assert_eq!(board.to_coords(31), (7, 6));
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
