use crate::{board_handler::BoardHandler, checkers::Checkers, player::Player, state::State};

pub struct GameManager;

impl GameManager {
    pub fn start(game: &mut Checkers) {
        println!("Red always goes first.");
        println!("You are the {} player.", game.human_player);

        while BoardHandler::count_player_pieces(&game.board, Player::Red) > 0
            && BoardHandler::count_player_pieces(&game.board, Player::White) > 0
        {
            if !BoardHandler::can_move_some_piece(&game.board, game.current_player) {
                game.end_turn();
                break;
            }

            game.print();
            if game.current_player == game.human_player {
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
                    match (text_io::try_read!("{}"), text_io::try_read!("{}")) {
                        (Ok(y), Ok(x)) => {
                            response = Some((y, x));
                            break;
                        }
                        _ => (),
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
                // do MinMax things
                todo!("bot logic");
            }
        }

        Self::end(None);
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
