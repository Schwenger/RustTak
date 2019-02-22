use crate::player::Color;
use crate::board::{Position, Direction, piece::PieceKind};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Move {
    pub action: Action,
    pub player: Color,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    /// Contains the start position and slide direction. The `vec` dictates how many pieces are
    /// carried over to the new field, i.e. if the original stack contains 4 pieces, all of them
    /// are supposed to be moved one to the right, and the top-most three one step further, then
    /// the `Action` should be: `Slide(source, Direction::East, vec![4,3]`
    Slide(Position, Direction, Vec<usize>),
    /// Contains the target position and the kind of piece that is supposed to be placed at the
    /// target.
    Place(Position, PieceKind),
}
