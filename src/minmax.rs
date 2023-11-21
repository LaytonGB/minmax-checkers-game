use std::collections::HashSet;

use crate::{board::Board, bot::Bot, checkers::Checkers, player::Player};

#[derive(Clone, Debug)]
pub struct MinMax;

impl Bot for MinMax {
    fn get_next_move(&self, game: &Checkers, depth_limit: Option<usize>) -> usize {
        self.next_move(game, depth_limit, false)
    }

    fn get_next_move_with_display(&self, game: &Checkers, depth_limit: Option<usize>) -> usize {
        self.next_move(game, depth_limit, true)
    }
}

impl MinMax {
    fn next_move(&self, game: &Checkers, depth_limit: Option<usize>, display: bool) -> usize {
        let mut game = game.clone();
        let bot_player = game.current_player();
        let mut visited = HashSet::new();
        self.evaluate_moves(
            &mut game,
            bot_player,
            depth_limit,
            0,
            i64::MIN,
            i64::MAX,
            display,
            &mut visited,
        )
        .1
        .expect("should always make some move")
    }

    fn evaluate_moves(
        &self,
        game: &mut Checkers,
        bot_player: Player,
        depth_limit: Option<usize>,
        depth: usize,
        mut alpha: i64,
        mut beta: i64,
        display_simulations: bool,
        visited_set: &mut HashSet<Board>,
    ) -> (i64, Option<usize>) {
        // base case: can't move (self or other) OR depth reached
        //      do: calculate score, return score and move
        // recurse case: not (win or lose)
        //      do: recurse for each valid move, return best

        let is_current_player = game.current_player() == bot_player; // bot always maximizing
        let mut best_score = if is_current_player {
            i64::MIN
        } else {
            i64::MAX
        };
        let board = game.board();
        if visited_set.contains(board) {
            return (best_score, None);
        }
        // visited_set.insert(board.clone());

        #[cfg(feature = "standalone")]
        if display_simulations {
            game.show_board();
        }

        let valid_moves: Vec<usize> = dbg!(game.selectable_positions().to_vec());
        let mut best_move = None;
        if valid_moves.is_empty() {
            let score: i64 = game.get_player_piece_count(bot_player) as i64
                - game.get_player_piece_count(bot_player.other()) as i64;
            (score, best_move)
        } else {
            for pos in valid_moves.into_iter() {
                game.make_a_move_from_api(pos);
                let new_score = self
                    .evaluate_moves(
                        game,
                        bot_player,
                        depth_limit,
                        depth + 1,
                        alpha,
                        beta,
                        display_simulations,
                        visited_set,
                    )
                    .0;
                if match is_current_player {
                    true => new_score > best_score,
                    false => new_score < best_score,
                } {
                    best_score = new_score;
                    best_move = Some(pos);
                }
                game.undo_last_move();
                match is_current_player {
                    true => alpha = alpha.max(best_score),
                    false => beta = beta.min(best_score),
                };
                if alpha >= beta {
                    break;
                }
            }
            (best_score, best_move)
        }
    }
}
