use crate::board::{Board, Position};
use crate::actions::{Move, Action};
use crate::simulator::Outcome;
use std::ops::Not;
use std::fmt::{Display, Formatter, Result as FmtResult, Debug};

mod command_line_human;

pub trait Player: Clone {
    fn welcome(&self, first: bool, opponent: &String);
    fn action_for(&self, board: &Board, opponent_action: Option<Action>) -> Action;
    fn first_action(&self, board: &Board) -> Position;
    fn accept_outcome(&self, outcome: &Outcome);
    fn name(&self) -> &String;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Blk,
}

impl Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            Color::Red => Color::Blk,
            Color::Blk => Color::Red,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Color::Red => write!(f, "Red"),
            Color::Blk => write!(f, "Black"),
        }
    }
}