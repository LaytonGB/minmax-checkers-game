use kar_checkers_minmax::{bot_choice::BotChoice, checkers::Checkers, player::Player};

fn main() {
    let mut game = Checkers::new(Some((Player::White, BotChoice::MinMax)));
    game.start();
}
