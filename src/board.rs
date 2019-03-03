pub(crate) mod piece;
mod position;

use self::piece::Stack;
pub use self::position::Position;
use std::ops::Index;
use crate::board::piece::Piece;
use crate::player::Color;
use crate::board::piece::PieceKind;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PiecesStash {
    stones: u16,
    caps: u16,
}

impl PiecesStash {
    pub(crate) fn for_board_size(n: usize) -> PiecesStash {
        let (stones, caps) = match n {
            3 => (10, 0),
            4 => (15, 0),
            5 => (21, 1),
            6 => (30, 1),
            8 => (50, 2),
            n => ((n as f32).powf(1.88) as u16, (n / 4) as u16), // The first rule seems about right, the second one not so much.
        };
        PiecesStash { stones, caps }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    board: Vec<Vec<Stack>>,
    red_pieces: PiecesStash,
    blk_pieces: PiecesStash,
}

//impl IndexMut<Position> for Board {
//    fn index_mut(&mut self, pos: Position) -> &mut Stack {
//        &mut self.board[pos.row][pos.col]
//    }
//}

impl Index<Position> for Board {
    type Output = Stack;
    fn index(&self, pos: Position) -> &Stack {
        &self.board[pos.row][pos.col]
    }
}

impl Board {

    fn mut_pos(&mut self, pos: Position) -> &mut Stack {
        &mut self.board[pos.row][pos.col]
    }

    pub fn new(size: usize) -> Board {
        let stash = PiecesStash::for_board_size(size);
        Board {
            board: vec![vec![Stack::empty(); size]; size],
            red_pieces: stash.clone(),
            blk_pieces: stash,
        }
    }

    pub fn valid_pos(&self, pos: Position) -> bool {
        let n = self.board.len();
        pos.row < n && pos.col < n
    }

    fn piece_count_mut(&mut self, c: Color, kind: PieceKind) -> &mut u16 {
        match (c, kind) {
            (Color::Blk, PieceKind::CapStone) => &mut self.blk_pieces.caps,
            (Color::Red, PieceKind::CapStone) => &mut self.red_pieces.caps,
            (Color::Blk, _) => &mut self.blk_pieces.stones,
            (Color::Red, _) => &mut self.red_pieces.stones,
        }
    }

    pub fn piece_count(&self, c: Color, kind: PieceKind) -> u16 {
        match (c, kind) {
            (Color::Blk, PieceKind::CapStone) => self.blk_pieces.caps,
            (Color::Red, PieceKind::CapStone) => self.red_pieces.caps,
            (Color::Blk, _) => self.blk_pieces.stones,
            (Color::Red, _) => self.red_pieces.stones,
        }
    }

    /// Places `piece` at the specified position.
    /// Panics if the position is invalid, occupied, or the player does not have a
    /// suitable piece left in their stash.
    pub fn place(&mut self, piece: Piece, at: Position) {
        assert!(self.valid_pos(at));
        assert!(self[at].is_empty());
        let left = self.piece_count_mut(piece.color, piece.kind);
        assert!(*left > 0);
        *left -= 1;
        *self.mut_pos(at) = Stack::from(piece);
    }

    /// Slides the `n` topmost pieces from `src` in `to` direction.
    /// Panics if:
    /// * `src` is not valid
    /// * the position in `to` direction of `src` does not exist
    /// * there are less than `n` pieces on `src`
    /// * the target position is occupied by a stack incompatible with the slid pieces.
    pub fn slide(&mut self, src: Position, to: Direction, n: usize) {
        let carried = self.mut_pos(src).take_off(n);
        *self.mut_pos(src.go(to)) += carried;
    }

    pub fn size(&self) -> usize {
        self.board.len()
    }

    #[cfg(test)]
    pub(crate) fn set_forcefully(&mut self, pos: Position, stack: Stack) {
        for piece in stack.iter() {
            *self.piece_count_mut(piece.color, piece.kind) -= 1;
        }
        *self.mut_pos(pos) = stack;
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
