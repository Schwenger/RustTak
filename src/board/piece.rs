
use crate::player::Color;
use std::ops::{AddAssign, SubAssign};
use crate::board::piece::PieceKind::CapStone;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub(crate) fn new(kind: PieceKind, color: Color) -> Piece {
        Piece { kind, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Stone,
    CapStone,
    StandingStone,
}

impl PieceKind {
    fn stackable(&self) -> bool {
        match self {
            PieceKind::Stone => true,
            PieceKind::StandingStone => false,
            PieceKind::CapStone => false,
        }
    }

    fn road(&self) -> bool {
        match self {
            PieceKind::Stone => true,
            PieceKind::StandingStone => false,
            PieceKind::CapStone => true,
        }
    }

    fn flattenable(&self) -> bool {
        match self {
            PieceKind::StandingStone => true,
            PieceKind::CapStone => false,
            PieceKind::Stone => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Stack {
    // The rightmost piece is the top piece.
    content: Vec<Piece>,
}

impl Stack {

    pub fn nth_piece(&self, n: usize) -> Piece {
        self.content[self.len() - 1 - n]
    }

    pub fn iter(&self) -> std::slice::Iter<Piece> {
        self.content.iter()
    }

    pub(crate) fn empty() -> Stack {
        Stack { content: Vec::new() }
    }

    pub(crate) fn is_road(&self) -> bool {
        self.top().map(|p| p.kind.road()).unwrap_or(false)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub(crate) fn compatible_with(&self, other: &Stack) -> bool {
        self.is_stackable() || self.is_flattenable() && other.is_flattening()
    }

    pub(crate) fn take_off(&mut self, n: usize) -> Stack {
        self.content.split_off(self.content.len() -  n).into()
    }

    pub(crate) fn peek_from_top(&self, n: usize) -> Stack {
        let new_content = self.content.split_at(self.content.len() - n).1;
        Vec::from(new_content).into()
    }

    pub(crate) fn color(&self) -> Option<Color> {
        self.top().map(|p| p.color)
    }

    pub(crate) fn len(&self) -> usize {
        self.content.len()
    }

    fn top(&self) -> Option<&Piece> {
        self.content.last()
    }

    fn is_atomic(&self) -> bool {
        self.content.len() == 1
    }

    fn is_flattening(&self) -> bool {
        if self.content.len() == 1 {
            self.top().map(|t| t.kind == PieceKind::CapStone).unwrap_or(false)
        } else {
            false
        }
    }

    fn is_flattenable(&self) -> bool {
        self.top().map(|p| p.kind.flattenable()).unwrap_or(true)
    }

    fn is_stackable(&self) -> bool {
        self.top().map(|p| p.kind.stackable()).unwrap_or(true)
    }

    fn flatten(&mut self) {
        if let Some(top) = self.top() {
            let new_top = match top.kind {
                PieceKind::StandingStone => Piece { kind: PieceKind::Stone, color: top.color },
                _ => return, // Short-circuit-ish
            };
            *self -= 1;
            *self += new_top; // TODO: We do not need to flatten again here.
        }
    }

    fn valid(&self) -> bool {
        if self.len() <= 1 {
            true
        } else {
            let tail = &self.content[0..(self.len() - 1)];
            tail.iter().all(|p| {
                PieceKind::Stone == p.kind
            })
        }
    }

}

impl AddAssign for Stack {
    fn add_assign(&mut self, mut other: Stack) {
        assert!(self.compatible_with(&other));
        self.flatten();
        self.content.extend(other.content.drain(..))
    }
}

impl AddAssign<Piece> for Stack {
    fn add_assign(&mut self, other: Piece) {
        self.add_assign(Stack::from(vec![other]));
    }
}

impl SubAssign<usize> for Stack {
    fn sub_assign(&mut self, n: usize) {
        let _ = self.take_off(n);
    }
}

impl From<Vec<Piece>> for Stack {
    fn from(v: Vec<Piece>) -> Stack {
        let res = Stack { content: v };
        assert!(res.valid());
        res
    }
}