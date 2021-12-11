use std::collections::HashMap;

#[derive(Debug)]
pub struct Board {
    positions: HashMap<BoardPosition, Player>
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum BoardPosition {
    A1,
    A2,
    A3,
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
}

fn is_winner_1st_row(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A1) &&
           positions.contains(&&BoardPosition::A2) &&
           positions.contains(&&BoardPosition::A3);
}

fn is_winner_2nd_row(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::B1) &&
           positions.contains(&&BoardPosition::B2) &&
           positions.contains(&&BoardPosition::B3);
}

fn is_winner_3rd_row(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::C1) &&
           positions.contains(&&BoardPosition::C2) &&
           positions.contains(&&BoardPosition::C3);
}

fn is_winner_1st_col(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A1) &&
           positions.contains(&&BoardPosition::B1) &&
           positions.contains(&&BoardPosition::C1);
}

fn is_winner_2nd_col(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A2) &&
           positions.contains(&&BoardPosition::B2) &&
           positions.contains(&&BoardPosition::C2);
}

fn is_winner_3rd_col(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A3) &&
           positions.contains(&&BoardPosition::B3) &&
           positions.contains(&&BoardPosition::C3);
}

fn is_winner_diagonal_desc(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A1) &&
           positions.contains(&&BoardPosition::B2) &&
           positions.contains(&&BoardPosition::C3);
}

fn is_winner_diagonal_asc(positions: &Vec<&BoardPosition>) -> bool {
    return positions.contains(&&BoardPosition::A3) &&
           positions.contains(&&BoardPosition::B2) &&
           positions.contains(&&BoardPosition::C1);
}

impl Board {
    fn new() -> Board {
        Self {
            positions: HashMap::with_capacity(9),
        }
    }

    fn is_position_available(&self, board_position: BoardPosition) -> bool {
        return self.positions.get(&board_position).is_none();
    }

    fn play(&mut self, player: Player, board_position: BoardPosition) -> bool {
        if !self.is_position_available(board_position) {
            return false;
        }
        return self.positions.insert(board_position, player).is_none();
    }

    fn is_winner(&self, player: Player) -> bool {
        let positions: Vec<&BoardPosition> = self.positions.iter()
                                                .filter_map(|kv| if kv.1 == &player { Some(kv.0) } else { None })
                                                .collect();
        return is_winner_1st_row(&positions) ||
               is_winner_2nd_row(&positions) ||
               is_winner_3rd_row(&positions) ||
               is_winner_1st_col(&positions) ||
               is_winner_2nd_col(&positions) ||
               is_winner_3rd_col(&positions) ||
               is_winner_diagonal_desc(&positions) ||
               is_winner_diagonal_asc(&positions);
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "  1   2   3\nA {a1} | {a2} | {a3}\n  {line}\nB {b1} | {b2} | {b3}\n  {line}\nC {c1} | {c2} | {c3}",
            line = "----------",
            a1 = self.positions.get(&BoardPosition::A1).to_string(),
            a2 = self.positions.get(&BoardPosition::A2).to_string(),
            a3 = self.positions.get(&BoardPosition::A3).to_string(),
            b1 = self.positions.get(&BoardPosition::B1).to_string(),
            b2 = self.positions.get(&BoardPosition::B2).to_string(),
            b3 = self.positions.get(&BoardPosition::B3).to_string(),
            c1 = self.positions.get(&BoardPosition::C1).to_string(),
            c2 = self.positions.get(&BoardPosition::C2).to_string(),
            c3 = self.positions.get(&BoardPosition::C3).to_string(),
        )
    }
}

impl BoardPosition {
    fn from(row: char, col: char) -> Option<BoardPosition> {
        match (row, col) {
            ('A', '1') => return Some(BoardPosition::A1),
            ('A', '2') => return Some(BoardPosition::A2),
            ('A', '3') => return Some(BoardPosition::A3),
            ('B', '1') => return Some(BoardPosition::B1),
            ('B', '2') => return Some(BoardPosition::B2),
            ('B', '3') => return Some(BoardPosition::B3),
            ('C', '1') => return Some(BoardPosition::C1),
            ('C', '2') => return Some(BoardPosition::C2),
            ('C', '3') => return Some(BoardPosition::C3),
            _ => return None
        }
    }
}
