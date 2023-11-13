use strum::EnumIter;

#[derive(Clone, Copy, PartialEq, EnumIter, Eq, Debug)]
pub enum Diagonal {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
