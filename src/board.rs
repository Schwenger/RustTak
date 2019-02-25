pub(crate) mod piece;

use self::piece::Stack;
use std::ops::{Index, IndexMut};

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
            Direction::North => Position { row: self.row + 1, col: self.col },
            Direction::South => Position { row: self.row - 1, col: self.col },
            Direction::East => Position { row: self.row, col: self.col + 1 },
            Direction::West => Position { row: self.row, col: self.col - 1 },
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

pub struct PosBoardIterator<'a> {
    row: usize,
    col: usize,
    board: &'a Board,
}

impl<'a> Iterator for PosBoardIterator<'a> {
    type Item = (Position, &'a Stack);
    fn next(&mut self) -> Option<(Position, &'a Stack)> {
        if self.row >= self.board.size() {
            return None;
        }
        let pos = Position::new(self.row, self.col);
        let current = &self.board[pos];
        self.col += 1;
        if self.col == self.board.size() {
            self.col = 0;
            self.row += 1;
        }
        Some((pos, current))
    }
}

pub struct BoardIterator<'a> {
    inner: PosBoardIterator<'a>,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = &'a Stack;
    fn next(&mut self) -> Option<&'a Stack> {
        self.inner.next().map(|(_pos, s)| s)
    }
}

impl<'a> PosBoardIterator<'a> {
    fn new(board: &Board) -> PosBoardIterator {
        PosBoardIterator { row: 0, col: 0, board }
    }
}

impl<'a> BoardIterator<'a> {
    fn new(board: &Board) -> BoardIterator {
        BoardIterator { inner: PosBoardIterator::new(board) }
    }

    pub fn with_pos(self) -> PosBoardIterator<'a> {
        self.inner
    }
}

impl Board {
    pub fn iter(&self) -> BoardIterator {
        BoardIterator::new(&self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
