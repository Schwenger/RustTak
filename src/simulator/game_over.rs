use crate::board::Board;
use crate::player::Color;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Outcome {
    pub result: MatchResult,
    pub board: Board,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MatchResult {
    Winner(Color),
    Tie,
}
