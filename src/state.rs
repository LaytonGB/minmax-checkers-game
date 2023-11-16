#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub enum State {
    #[default]
    Selecting,
    Moving(usize),
    Chaining(Vec<usize>),
}
