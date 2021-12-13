include!("board.rs");
include!("player.rs");

#[derive(Debug)]
struct State {
    board: Board,
    current_player: Player,
    winner: Option<Player>,
    moves: u8,
}

impl State {
    fn new() -> Self {
        Self::with(Player::X)
    }

    fn with(initial_player: Player) -> Self {
        Self {
            board: Board::new(),
            current_player: initial_player,
            winner: None,
            moves: 0,
        }
    }

    fn play(&mut self, position: BoardPosition) -> bool {
        if !self.board.play(self.current_player, position) {
            return false;
        }
        if self.board.is_winner(self.current_player) {
            self.winner = Some(self.current_player);
        } else {
            match self.current_player {
                Player::X => self.current_player = Player::O,
                Player::O => self.current_player = Player::X,
            }
        }
        self.moves += 1;
        return true;
    }
}
