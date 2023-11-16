// BUG taking a piece that can capture ends turn early, which causes a crash

use kar_checkers_minmax::checkers::Checkers;
use kar_checkers_minmax::game_manager::GameManager;
use kar_checkers_minmax::player::Player;

const BOARD_SIZE: usize = 8;

fn main() {
    // let mut game = Checkers::new(BOARD_SIZE, Some(Player::White));
    let mut game = Checkers::default();
    GameManager::start(&mut game);
}
