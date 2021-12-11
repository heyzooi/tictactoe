include!("state.rs");

pub struct Game {
    state: State,
    user_input: Box<dyn UserInput>,
}

pub trait UserInput {
    fn read_from_keyboard(&mut self) -> std::result::Result<String, std::io::Error>;
}

struct ReadFromStdIn {
}

impl ReadFromStdIn {
    fn new() -> Self {
        ReadFromStdIn {
        }
    }
}

impl UserInput for ReadFromStdIn {
    fn read_from_keyboard(&mut self) -> std::result::Result<String, std::io::Error> {
        let mut buffer = String::new();
        return std::io::stdin().read_line(&mut buffer).map(|_| buffer);
    }
}

impl Game {

    pub fn new() -> Self {
        Self {
            state: State::new(),
            user_input: Box::new(ReadFromStdIn::new()),
        }
    }

    #[allow(dead_code)]
    pub fn with(user_input: Box<dyn UserInput>) -> Self {
        return Self {
            state: State::new(),
            user_input: user_input,
        }
    }

    fn read_next_move(&mut self) -> Option<BoardPosition> {
        println!("Player {}'s turn: ", self.state.current_player);
        let result = self.user_input.as_mut().read_from_keyboard().unwrap();
        let move_input = result.trim();
        if move_input.len() == 2 {
            let move_input_upper = move_input.to_uppercase();
            let mut chars = move_input_upper.chars();
            let row = chars.nth(0).unwrap();
            let col = chars.nth(0).unwrap();
            return BoardPosition::from(row, col);
        } else {
            return None;
        }
    }

    fn play_position(&mut self, position: BoardPosition) {
        if !self.state.play(position) {
            println!("Position not available!");
        }
    }

    pub fn play(&mut self) {
        while self.state.winner.is_none() {
            println!("{}", self.state.board);

            match self.read_next_move() {
                Some(position) => self.play_position(position),
                None => println!("Invalid position!"),
            }
            println!();
        }

        println!("{}", self.state.board);
        println!("Winner is: Player {}", self.state.winner.unwrap());
    }
}
