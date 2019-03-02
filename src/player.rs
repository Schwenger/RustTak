use crate::actions::Action;
use crate::board::{Board, Position};
use crate::simulator::game_over::Outcome;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::stdin;
use std::ops::Not;

mod command_line_human;

use self::command_line_human::CommandLineHuman;

#[derive(Debug)]
pub struct HumanPlayer {
    name: String,
}

impl HumanPlayer {
    pub fn command_line_interface() -> HumanPlayer {
        let mut player = HumanPlayer { name: String::new() };
        println!("Welcome to Tak!\nWhat's your name?");
        let _ = stdin().read_line(&mut player.name);
        player
    }

    pub fn graphical_interface() -> HumanPlayer {
        unimplemented!()
    }
}

impl PlayerBuilder<CommandLineHuman> for HumanPlayer {
    fn setup(self, board_size: usize, color: Color, first: bool) -> CommandLineHuman {
        CommandLineHuman::new(self.name, board_size, color, first)
    }
}

pub trait PlayerBuilder<T: Player>: Sized {
    fn setup(self, board_size: usize, color: Color, first: bool) -> T;
}

pub trait Player: Clone {
    fn welcome(&mut self, opponent: &String);
    fn action_for(&mut self, board: &Board, opponent_action: Option<Action>) -> Action;
    fn first_action(&mut self, board: &Board) -> Position;
    fn accept_outcome(&mut self, outcome: &Outcome);
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
