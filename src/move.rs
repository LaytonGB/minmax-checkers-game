use crate::piece::Piece;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Move {
    start: usize,
    end: usize,
    started_king: bool,
    ended_king: bool,
    capture: Option<(usize, Piece)>,
}

impl Move {
    pub fn new_select(position: usize, is_king: bool) -> Self {
        Self {
            start: position,
            end: position,
            started_king: is_king,
            ended_king: is_king,
            capture: None,
        }
    }

    pub fn new_move(start: usize, end: usize, starts_as_king: bool, ends_as_king: bool) -> Self {
        Self {
            start,
            end,
            started_king: starts_as_king,
            ended_king: ends_as_king,
            capture: None,
        }
    }

    pub fn new_capture(
        start: usize,
        end: usize,
        starts_as_king: bool,
        ends_as_king: bool,
        capture_position: usize,
        capture_piece: Piece,
    ) -> Self {
        Self {
            start,
            end,
            started_king: starts_as_king,
            ended_king: ends_as_king,
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
    pub fn capture(&self) -> Option<(usize, Piece)> {
        self.capture
    }

    #[inline]
    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    pub fn started_king(&self) -> bool {
        self.started_king
    }

    pub fn ended_king(&self) -> bool {
        self.ended_king
    }
}
