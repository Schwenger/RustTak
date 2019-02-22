
pub(crate) mod piece;

use self::piece::Stack;
use std::ops::{IndexMut, Index};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {

    pub fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }

    pub(crate) fn go(self, dir: Direction) -> Position {
        match dir {
            Direction::North => Position{ row: self.row + 1, col: self.col },
            Direction::South => Position{ row: self.row - 1, col: self.col },
            Direction::East  => Position{ row: self.row, col: self.col + 1 },
            Direction::West  => Position{ row: self.row, col: self.col - 1 },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    board: Vec<Vec<Stack>>,
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, pos: Position) -> &mut Stack {
        &mut self.board[pos.row][pos.col]
    }
}

impl Index<Position> for Board {
    type Output = Stack;
    fn index(&self, pos: Position) -> &Stack {
        &self.board[pos.row][pos.col]
    }
}

impl Board {
    pub(crate) fn new(size: usize) -> Board {
        Board { board: vec![vec![Stack::empty(); size]; size] }
    }

    pub(crate) fn remove_from(&mut self, pos: Position) -> Stack {
        let res = self[pos].clone();
        self[pos] = Stack::empty();
        res
    }

    pub(crate) fn empty(&mut self, pos: Position) {
        self[pos] = Stack::empty();
    }

    pub fn valid_pos(&self, pos: Position) -> bool {
        let n = self.board.len();
        pos.row < n && pos.col < n
    }

    pub fn size(&self) -> usize {
        self.board.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}