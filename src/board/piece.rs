use crate::player::Color;
use std::ops::{AddAssign, SubAssign};

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
    fn stackable(self) -> bool {
        match self {
            PieceKind::Stone => true,
            PieceKind::StandingStone => false,
            PieceKind::CapStone => false,
        }
    }

    fn road(self) -> bool {
        match self {
            PieceKind::Stone => true,
            PieceKind::StandingStone => false,
            PieceKind::CapStone => true,
        }
    }

    fn flattenable(self) -> bool {
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

    pub fn is_atomic(&self) -> bool {
        self.content.len() == 1
    }

    pub fn is_road(&self) -> bool {
        self.top().map(|p| p.kind.road()).unwrap_or(false)
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn compatible_with(&self, other: &Stack) -> bool {
        self.is_stackable() || self.is_flattenable() && other.is_flattening()
    }

    pub fn color(&self) -> Option<Color> {
        self.top().map(|p| p.color)
    }

    pub fn take_off(&mut self, n: usize) -> Stack {
        self.content.split_off(self.content.len() - n).into()
    }

    pub fn empty() -> Stack {
        Stack { content: Vec::new() }
    }

    pub fn peek_from_top(&self, n: usize) -> Stack {
        let new_content = self.content.split_at(self.content.len() - n).1;
        Vec::from(new_content).into()
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_flattening(&self) -> bool {
        if self.content.len() == 1 {
            self.top().map(|t| t.kind == PieceKind::CapStone).unwrap_or(false)
        } else {
            false
        }
    }

    pub fn is_flattenable(&self) -> bool {
        self.top().map(|p| p.kind.flattenable()).unwrap_or(true)
    }

    pub fn is_stackable(&self) -> bool {
        self.top().map(|p| p.kind.stackable()).unwrap_or(true)
    }

    pub fn flatten(&mut self) {
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
            tail.iter().all(|p| PieceKind::Stone == p.kind)
        }
    }

    fn top(&self) -> Option<&Piece> {
        self.content.last()
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

#[cfg(test)]
mod tests {

    use super::{Piece, Stack};
    use crate::board::piece::PieceKind;
    use crate::player::{
        Color,
        Color::{Blk, Red},
    };
    use crate::test_util::*;

    #[test]
    fn take_off() {
        let mut subject = stack_with_cap_rbr();
        assert_eq!(subject.take_off(0), Stack::from(Vec::new()));
        assert_eq!(subject, stack_with_cap_rbr());
        assert_eq!(subject.take_off(1), single_cap(Red));
        assert_eq!(subject, Stack::from(vec![stone(Red), stone(Blk)]));
        assert_eq!(subject.take_off(1), single_stone(Blk));
        assert_eq!(subject, single_stone(Red));
        assert_eq!(subject.take_off(1), single_stone(Red));
        assert_eq!(subject, Stack::from(Vec::new()));
    }

    #[test]
    fn flattening_standing_success() {
        let mut control = stack_with_standing_rbr();
        let mut subject = control.clone();
        let top = single_cap(Red);
        assert!(subject.compatible_with(&top));
        subject += top.clone();
        assert_eq!(subject.take_off(top.len()), top);
        control -= 1;
        control += Piece::new(PieceKind::Stone, Color::Red);
        assert_eq!(subject, control);
    }

    #[test]
    fn flattening_standing_fail() {
        let bot = stack_with_standing_rbr();
        let top = stack_with_cap_rbr();
        assert!(!bot.compatible_with(&top));
    }

    #[test]
    fn flattening_capstone_fail() {
        let bot = stack_with_standing_rbr();
        let top = stack_with_cap_rbr();
        assert!(!bot.compatible_with(&top));
    }

    #[test]
    fn test_compatible_stack() {
        let bot = stack_with_cap_rbr();
        let top = single_stone(Red);
        assert!(!bot.compatible_with(&top));
    }

    #[test]
    fn test_compatible_solo() {
        let bot = single_cap(Red);
        let top = single_stone(Red);
        assert!(!bot.compatible_with(&top));
    }

    #[test]
    fn test_compatible_stone() {
        let bot = single_stone(Red);
        let top = single_cap(Red);
        assert!(bot.compatible_with(&top));
    }

    #[test]
    fn test_compatible_standing() {
        let bot = single_stone(Red);
        let top = single_standing(Red);
        assert!(bot.compatible_with(&top));
    }

    #[test]
    fn test_nth() {
        let mut s0 = stack_stone_rbr();
        let s1 = stack_with_standing_rbr();
        let s2 = single_cap(Blk);
        s0 += s1;
        s0 += s2;
        // Red Blk Red Red Blk Red Blk
        // Sto Sto Sto Sto Sto Sto Cap
        // 6   5   4   3   2   1   0
        assert_eq!(s0.nth_piece(0), cap_stone(Blk));
        assert_eq!(s0.nth_piece(1), stone(Red));
        assert_eq!(s0.nth_piece(2), stone(Blk));
        assert_eq!(s0.nth_piece(3), stone(Red));
        assert_eq!(s0.nth_piece(4), stone(Red));
        assert_eq!(s0.nth_piece(5), stone(Blk));
        assert_eq!(s0.nth_piece(6), stone(Red));
    }

}
