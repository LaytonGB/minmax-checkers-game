use crate::player::Player;

#[derive(Debug)]
pub enum Turn {
    Captures(Vec<(usize, usize)>),
    Moves(Vec<usize>),
}

#[derive(Default, Debug)]
pub struct History(Vec<(Player, Turn)>);
