#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Selecting,
    Moving,
    Chaining,
}
