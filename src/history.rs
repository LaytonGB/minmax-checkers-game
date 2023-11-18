use crate::{player::Player, r#move::Move};

#[derive(Debug)]
pub struct History(pub Vec<(Player, Vec<Move>)>);

impl History {
    pub fn get_last_player(&self) -> Option<Player> {
        self.0.last().and_then(|t| Some(t.0))
    }

    pub fn get_last_turn(&self) -> Option<&Vec<Move>> {
        self.0.last().and_then(|t| Some(&t.1))
    }

    pub fn get_last_move(&self) -> Option<&Move> {
        self.0.last().and_then(|t| t.1.last())
    }

    pub fn last_move_was_capture(&self) -> bool {
        if let Some(last_move) = self.get_last_move() {
            last_move.is_capture()
        } else {
            false
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self(Default::default())
    }
}
