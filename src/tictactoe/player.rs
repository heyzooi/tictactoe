#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

trait OptionPlayer {
    fn to_string(&self) -> String;
}

impl OptionPlayer for Option<&Player> {
    fn to_string(&self) -> String {
        match self {
            Some(player) => format!("{}", player),
            None => String::from(" "),
        }
    }
}
