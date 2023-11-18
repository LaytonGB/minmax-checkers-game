#[derive(Debug)]
pub enum Turn {
    Captures(Vec<(usize, usize)>),
    Moves(Vec<usize>),
}
