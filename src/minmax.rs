use crate::{bot::Bot, checkers::Checkers, player::Player};

#[derive(Clone, Debug)]
pub struct MinMax;

impl Bot for MinMax {
    fn get_next_move(&self, game: &Checkers, depth_limit: Option<usize>) -> usize {
        let mut game = game.clone();
        let bot_player = game.current_player();
        self.evaluate_moves(&mut game, bot_player, depth_limit, 0)
            .1
            .expect("should always make some move")
    }
}

impl MinMax {
    fn evaluate_moves(
        &self,
        game: &mut Checkers,
        bot_player: Player,
        depth_limit: Option<usize>,
        depth: usize,
    ) -> (usize, Option<usize>) {
        // base case: can't move (self or other) OR depth reached
        //      do: calculate score, return score and move
        // recurse case: not (win or lose)
        //      do: recurse for each valid move, return best
        let valid_moves: Vec<usize> = game.selectable_positions().to_vec();
        let is_current_player = game.current_player() == bot_player;
        let mut best_score = if is_current_player {
            usize::MIN
        } else {
            usize::MAX
        };
        let mut best_move = None;
        if valid_moves.is_empty() {
            let score = game.get_player_piece_count(bot_player)
                - game.get_player_piece_count(bot_player.other());
            (score, best_move)
        } else {
            for pos in valid_moves.into_iter() {
                game.make_a_move_from_api(pos);
                let new_score = self
                    .evaluate_moves(game, bot_player, depth_limit, depth + 1)
                    .0;
                if match is_current_player {
                    true => new_score > best_score,
                    false => new_score < best_score,
                } {
                    best_score = new_score;
                    best_move = Some(pos);
                }
                game.undo_last_move();
            }
            (best_score, best_move)
        }
    }
}
