#[cfg(test)]
mod tests {
    include!("tictactoe/game.rs");

    #[test]
    fn test_inital_current_player() {
        let game = Game::new();
        assert_eq!(game.state.current_player, Player::X);
    }

    #[test]
    fn test_player_turn() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A1), true);
        assert_eq!(board.play(Player::X, BoardPosition::A1), false);
    }

    #[test]
    fn test_position_unavailable() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A1), true);
        assert_eq!(board.play(Player::O, BoardPosition::A1), false);
    }

    #[test]
    fn test_winner_1st_row() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A1), true);
        assert_eq!(board.play(Player::X, BoardPosition::A2), true);
        assert_eq!(board.play(Player::X, BoardPosition::A3), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_2nd_row() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::B1), true);
        assert_eq!(board.play(Player::X, BoardPosition::B2), true);
        assert_eq!(board.play(Player::X, BoardPosition::B3), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_3rd_row() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::C1), true);
        assert_eq!(board.play(Player::X, BoardPosition::C2), true);
        assert_eq!(board.play(Player::X, BoardPosition::C3), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_1st_col() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A1), true);
        assert_eq!(board.play(Player::X, BoardPosition::B1), true);
        assert_eq!(board.play(Player::X, BoardPosition::C1), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_2nd_col() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A2), true);
        assert_eq!(board.play(Player::X, BoardPosition::B2), true);
        assert_eq!(board.play(Player::X, BoardPosition::C2), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_3rd_col() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A3), true);
        assert_eq!(board.play(Player::X, BoardPosition::B3), true);
        assert_eq!(board.play(Player::X, BoardPosition::C3), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_diagonal_desc() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A1), true);
        assert_eq!(board.play(Player::X, BoardPosition::B2), true);
        assert_eq!(board.play(Player::X, BoardPosition::C3), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_winner_diagonal_asc() {
        let mut board = Board::new();
        assert_eq!(board.play(Player::X, BoardPosition::A3), true);
        assert_eq!(board.play(Player::X, BoardPosition::B2), true);
        assert_eq!(board.play(Player::X, BoardPosition::C1), true);
        assert_eq!(board.is_winner(Player::X), true);
        assert_eq!(board.is_winner(Player::O), false);
    }

    #[test]
    fn test_board_position() {
        assert_eq!(BoardPosition::from('A', '1'), Some(BoardPosition::A1));
    }

    #[test]
    fn test_game_board_initial_state() {
        let game = Game::new();
        assert_eq!(game.state.board.positions.get(&BoardPosition::A1), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::A2), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::A3), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::B1), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::B2), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::B3), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::C1), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::C2), None);
        assert_eq!(game.state.board.positions.get(&BoardPosition::C3), None);
    }

    struct MockUserInput {
        offset: usize,
        inputs: Vec<String>,
    }

    impl MockUserInput {
        fn new(inputs: Vec<String>) -> Self {
            Self {
                offset: 0,
                inputs: inputs
            }
        }
    }
    
    impl UserInput for MockUserInput {
        fn read_from_keyboard(&mut self) -> std::result::Result<String, std::io::Error> {
            let value = self.inputs[self.offset].clone();
            self.offset += 1;
            return Ok(value);
        }
    }

    #[test]
    fn test_game_play1() {
        let mock = Box::new(MockUserInput::new(
            vec![
                String::from("A1"),
                String::from("B1"),
                String::from("A2"),
                String::from("B2"),
                String::from("A3"),
            ],
        ));
        let mut game = Game::with(mock);
        game.play();
        assert_eq!(game.state.winner, Some(Player::X));
        assert_eq!(game.state.moves, 5);
        assert_eq!(game.state.board.positions.get(&BoardPosition::A1), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A2), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A3), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B2), Some(&Player::O));
    }

    #[test]
    fn test_game_play2() {
        let mock = Box::new(MockUserInput::new(
            vec![
                String::from("A1"),
                String::from("A2"),
                String::from("A3"),
                String::from("B1"),
                String::from("B2"),
                String::from("B3"),
                String::from("C1"),
            ],
        ));
        let mut game = Game::with(mock);
        game.play();
        assert_eq!(game.state.winner, Some(Player::X));
        assert_eq!(game.state.moves, 7);
        assert_eq!(game.state.board.positions.get(&BoardPosition::A1), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A2), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A3), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B2), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B3), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C1), Some(&Player::X));
    }

    #[test]
    fn test_game_play3() {
        let mock = Box::new(MockUserInput::new(
            vec![
                String::from("B2"),
                String::from("A1"),
                String::from("A3"),
                String::from("C1"),
                String::from("C3"),
                String::from("B1"),
            ],
        ));
        let mut game = Game::with(mock);
        game.play();
        assert_eq!(game.state.winner, Some(Player::O));
        assert_eq!(game.state.moves, 6);
        assert_eq!(game.state.board.positions.get(&BoardPosition::B2), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A3), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C3), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B1), Some(&Player::O));
    }

    #[test]
    fn test_game_play_game_over_no_winner() {
        /*
         O | X | X
         X | X | O
         O | O | X
        */
        let mock = Box::new(MockUserInput::new(
            vec![
                String::from("B2"),
                String::from("A1"),
                String::from("A3"),
                String::from("C1"),
                String::from("B1"),
                String::from("B3"),
                String::from("A2"),
                String::from("C2"),
                String::from("C3"),
            ],
        ));
        let mut game = Game::with(mock);
        game.play();
        assert_eq!(game.state.winner, None);
        assert_eq!(game.state.moves, 9);
        assert_eq!(game.state.board.positions.get(&BoardPosition::B2), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A3), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C1), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B1), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::B3), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::A2), Some(&Player::X));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C2), Some(&Player::O));
        assert_eq!(game.state.board.positions.get(&BoardPosition::C3), Some(&Player::X));
    }
}
