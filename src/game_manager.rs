use text_io::Error;

use crate::{board_handler::BoardHandler, checkers::Checkers, player::Player};

pub struct GameManager;

impl GameManager {
    pub fn start(game: &mut Checkers) {
        while BoardHandler::count_player_pieces(&game.board, Player::Red) > 0
            && BoardHandler::count_player_pieces(&game.board, Player::White) > 0
        {
            game.print();
            if game.current_player == game.human_player {
                let mut responses: Option<(usize, usize)> = None;
                while responses.is_none() {
                    println!("Make your next move in format: Row <space> Column");
                    match (text_io::try_read!("{}"), text_io::try_read!("{}")) {
                        (Ok(y), Ok(x)) => {
                            responses = Some((y, x));
                            break;
                        }
                        _ => (),
                    }
                }
                println!(
                    "RESPONSE: {} {}",
                    responses.as_ref().unwrap().0,
                    responses.unwrap().1
                )
            } else {
                // do MinMax things
            }
        }
    }
}
