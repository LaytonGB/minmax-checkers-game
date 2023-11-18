use crate::piece::Piece;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Move {
    start: usize,
    end: usize,
    capture: Option<(usize, Piece)>,
}

impl Move {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            capture: None,
        }
    }

    pub fn capture(
        start: usize,
        end: usize,
        capture_position: usize,
        capture_piece: Piece,
    ) -> Self {
        Self {
            start,
            end,
            capture: Some((capture_position, capture_piece)),
        }
    }

    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    #[inline]
    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }
}
