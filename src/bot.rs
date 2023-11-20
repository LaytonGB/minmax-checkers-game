use std::fmt::Debug;

use crate::checkers::Checkers;

pub trait Bot: Debug {
    fn get_next_move(&self, game: &Checkers, depth_limit: Option<usize>) -> usize;
}
