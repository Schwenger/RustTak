use crate::board::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    pub fn go(self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position { row: self.row + 1, col: self.col },
            Direction::South => Position { row: self.row - 1, col: self.col },
            Direction::East => Position { row: self.row, col: self.col + 1 },
            Direction::West => Position { row: self.row, col: self.col - 1 },
        }
    }
}