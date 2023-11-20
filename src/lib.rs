pub mod checkers;

#[cfg(feature = "standalone")]
pub(crate) mod io;

pub(crate) mod board;
pub(crate) mod bot;
pub(crate) mod cache;
pub(crate) mod constants;
pub(crate) mod history;
pub(crate) mod minmax;
pub(crate) mod r#move;
pub(crate) mod piece;
pub(crate) mod player;
