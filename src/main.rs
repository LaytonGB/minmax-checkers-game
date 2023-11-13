use kar_checkers_minmax::checkers::Checkers;
use kar_checkers_minmax::player::Player;

const BOARD_SIZE: usize = 8;
const BOARD_SIZE_SQUARED_HALVED: usize = BOARD_SIZE.pow(2) / 2;

fn main() {
    let game = dbg!(Checkers::<BOARD_SIZE_SQUARED_HALVED>::new(Player::Red));
}
