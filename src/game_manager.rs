use crate::{board_handler::BoardHandler, checkers::Checkers, player::Player, state::State};

pub struct GameManager;

impl GameManager {
    pub fn start(game: &mut Checkers) {
        println!("Red always goes first.");
        if let Some(bot_player) = game.bot_player {
            println!("You are the {} player.", bot_player.other());
        }

        while BoardHandler::count_player_pieces(&game.board, Player::Red) > 0
            && BoardHandler::count_player_pieces(&game.board, Player::White) > 0
        {
            if !(Self::can_move(game)) {
                println!("DEBUG: calling game.end_turn() from GameManager");
                game.end_turn();
            }

            game.print();
            if game
                .bot_player
                .and_then(|bot_player| Some(bot_player != game.current_player))
                .unwrap_or(true)
            {
                let mut response: Option<(usize, usize)> = None;
                while response.is_none() {
                    match game.state {
                        State::Selecting => println!("Select a piece to move:"),
                        State::Moving(start_position) => println!(
                            "You are moving the piece at {:?}.\nWhere are you moving the piece to?\nValid moves are {:?}.",
                            game.board.to_coord(start_position),
                            BoardHandler::get_valid_moves(&game.board, start_position).iter().map(|(_,p)| game.board.to_coord(*p)).collect::<Vec<_>>()
                        ),
                        State::Chaining(ref past_positions) => println!(
                            "You are moving the piece at {:?}.\nWhere are you moving the piece to?\nValid moves are {:?}",
                            game.board.to_coord(*past_positions.last().unwrap()),
                            BoardHandler::get_valid_moves(&game.board, *past_positions.last().unwrap()).iter().map(|(_,p)| game.board.to_coord(*p)).collect::<Vec<_>>()
                        ),
                    };
                    if let (Ok(y), Ok(x)) = (text_io::try_read!("{}"), text_io::try_read!("{}")) {
                        response = Some((y, x));
                    }
                }

                let position = game.board.to_position(response.unwrap());
                if let Err(e) = match game.state {
                    State::Selecting => game.select_piece(position),
                    State::Moving(_) => game.move_piece(position),
                    State::Chaining(_) => game.move_piece(position),
                } {
                    println!("\nERROR: {}\n", e);
                }
            } else {
                // TODO bot stuff
                todo!("bot logic");
            }
        }

        Self::end(None);
    }

    fn can_move(game: &Checkers) -> bool {
        match game.state {
            State::Selecting => {
                (&BoardHandler::movable_pieces_for_player(&game.board, game.current_player)).len()
                    > 0
            }
            State::Moving(position) => {
                BoardHandler::get_valid_moves(&game.board, position).len() > 0
            }
            State::Chaining(ref past_positions) => {
                let position = *past_positions.last().unwrap();
                BoardHandler::get_valid_captures(&game.board, position)
                    .unwrap_or(Vec::new())
                    .len()
                    > 0
            }
        }
    }

    pub fn end(winner: Option<Player>) {
        if let Some(winner) = winner {
            println!(
                "\n!!! WINNER !!!\nCongratulations to the {} player!",
                winner
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_move() {
        let mut game = Checkers::default();

        assert!(GameManager::can_move(&game));

        game.select_piece(8).ok();
        game.move_piece(12).ok();

        assert!(!GameManager::can_move(dbg!(&game)));

        game.end_turn();

        assert!(GameManager::can_move(&game));

        game.select_piece(21).ok();
        game.move_piece(16).ok();

        assert!(!GameManager::can_move(&game));

        game.end_turn();

        assert!(GameManager::can_move(&game));

        game.select_piece(12).ok();
        game.move_piece(21).ok();

        assert!(!GameManager::can_move(&game));
    }
}
