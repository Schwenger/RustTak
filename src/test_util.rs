use crate::board::piece::{Piece, PieceKind, Stack};
use crate::player::{Color, Color::*};

#[allow(dead_code)]
pub(crate) fn stone(c: Color) -> Piece {
    Piece::new(PieceKind::Stone, c)
}

#[allow(dead_code)]
pub(crate) fn cap_stone(c: Color) -> Piece {
    Piece::new(PieceKind::CapStone, c)
}

#[allow(dead_code)]
pub(crate) fn standing(c: Color) -> Piece {
    Piece::new(PieceKind::StandingStone, c)
}

#[allow(dead_code)]
pub(crate) fn single_stone(c: Color) -> Stack {
    Stack::from(vec![stone(c)])
}

#[allow(dead_code)]
pub(crate) fn single_standing(c: Color) -> Stack {
    Stack::from(vec![standing(c)])
}

#[allow(dead_code)]
pub(crate) fn single_cap(c: Color) -> Stack {
    Stack::from(vec![cap_stone(c)])
}

#[allow(dead_code)]
pub(crate) fn stack_stone_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), stone(Red)])
}

#[allow(dead_code)]
pub(crate) fn stack_with_cap_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), cap_stone(Red)])
}

#[allow(dead_code)]
pub(crate) fn stack_with_standing_rbr() -> Stack {
    Stack::from(vec![stone(Red), stone(Blk), standing(Red)])
}
