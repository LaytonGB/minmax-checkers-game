use crate::{player::Player, r#move::Move};

#[derive(Debug)]
pub struct History(pub Vec<(Player, bool, Vec<Move>, bool)>);

impl History {
    pub fn get_last_player(&self) -> Option<Player> {
        self.0.last().and_then(|t| Some(t.0))
    }

    pub fn get_last_turn(&self) -> Option<&Vec<Move>> {
        self.0.last().and_then(|t| Some(&t.2))
    }

    pub fn pop_last_turn(&mut self) -> Option<Vec<Move>> {
        self.0.pop().and_then(|t| Some(t.2))
    }

    pub fn get_last_move(&self) -> Option<&Move> {
        self.0.last().and_then(|t| t.2.last())
    }

    pub fn started_last_turn_as_king(&self) -> bool {
        self.0.last().and_then(|t| Some(t.1)).unwrap_or(false)
    }

    pub fn ended_last_turn_as_king(&self) -> bool {
        self.0.last().and_then(|t| Some(t.3)).unwrap_or(false)
    }

    pub fn last_move_was_capture(&self) -> bool {
        if let Some(last_move) = self.get_last_move() {
            last_move.is_capture()
        } else {
            false
        }
    }

    pub fn push(&mut self, player: Player, started_as_king: bool, r#move: Move, is_king: bool) {
        if self.get_last_player().unwrap_or(player.other()) == player {
            self.0.last_mut().map(|(_, _, moves, ended_as_king)| {
                moves.push(r#move);
                *ended_as_king = is_king;
            });
        } else {
            self.0
                .push((player, started_as_king, vec![r#move], is_king));
        }
    }
}

impl Default for History {
    fn default() -> Self {
        Self(Default::default())
    }
}
