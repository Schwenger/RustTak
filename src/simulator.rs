use crate::board::Board;
use crate::player::{Color, Player};
use crate::move_logic::MoveLogic;
use crate::actions::Move;

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

pub struct Simulator<R, B> {
    logic: MoveLogic,
    red: R,
    blk: B,
}

impl<R: Player, B: Player> Simulator<R, B> {

    /// Red always starts!
    pub fn new(red: R, blk: B, size: usize) -> Simulator<R, B> {
        Simulator { logic: MoveLogic::new(size), red, blk }
    }

    pub fn start(mut self) -> Outcome {
        self.red.welcome(&self.blk.name());
        self.blk.welcome(&self.red.name());

        let pos_red = self.red.first_action(&self.logic.peek());
        self.logic.first_turn(pos_red, Color::Red);
        let pos_black = self.blk.first_action(&self.logic.peek());
        self.logic.first_turn(pos_black, Color::Blk);

        let mut next = Color::Red;
        loop {
            if let Some(outcome) = self.next_move(next) {
                self.game_over(&outcome);
                return outcome
            } else {
                next = !next;
            }
        }
    }

    fn next_move(&mut self, c: Color) -> Option<Outcome> {
        let last = self.logic.last_applied_move.as_ref().map(|m| m.action.clone());
        let action = match c {
            Color::Red => self.red.action_for(self.logic.peek(), last),
            Color::Blk => self.blk.action_for(self.logic.peek(), last),
        };
        let mv = Move { action, player: c};
        self.logic.apply(mv)
    }

    fn game_over(&mut self, outcome: &Outcome) {
        self.red.accept_outcome(&outcome);
        self.blk.accept_outcome(&outcome)
    }

}