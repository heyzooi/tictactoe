mod tictactoe;
include!("tests.rs");

fn main() {
    let mut game = tictactoe::Game::new();
    game.play();
}
