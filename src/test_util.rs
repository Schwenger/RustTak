use crate::player::{Color, Color::*};
use crate::board::piece::{Piece, PieceKind, Stack};

pub(crate) fn stone(c: Color) -> Piece {
    Piece::new(PieceKind::Stone, c)
}

pub(crate) fn cap_stone(c: Color) -> Piece {
    Piece::new(PieceKind::CapStone, c)
}

pub(crate) fn standing(c: Color) -> Piece {
    Piece::new(PieceKind::StandingStone, c)
}

pub(crate) fn single_stone(c: Color) -> Stack {
    Stack::from(vec![stone(c)])
}

pub(crate) fn single_standing(c: Color) -> Stack {
    Stack::from(vec![standing(c)])
}

pub(crate) fn single_cap(c: Color) -> Stack {
    Stack::from(vec![cap_stone(c)])
}

pub(crate) fn stack_stone_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), stone(Red)])
}

pub(crate) fn stack_with_cap_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), cap_stone(Red)])
}

pub(crate) fn stack_with_standing_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), standing(Red)])
}