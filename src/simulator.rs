use crate::actions::Move;
use crate::player::{Color, Player};

pub mod game_over;
mod logic;

use self::game_over::Outcome;
use self::logic::Logic;
use crate::player::PlayerBuilder;

pub struct Simulator<R, B> {
    logic: Logic,
    red: R,
    blk: B,
}

impl<R: Player, B: Player> Simulator<R, B> {
    /// Red always starts!
    pub fn new<X, Y>(red: X, blk: Y, size: usize) -> Simulator<R, B>
    where
        X: PlayerBuilder<R>,
        Y: PlayerBuilder<B>,
    {
        let red = red.setup(size, Color::Red, true);
        let blk = blk.setup(size, Color::Blk, false);
        Simulator { logic: Logic::new(size), red, blk }
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
                return outcome;
            } else {
                next = !next;
            }
        }
    }

    fn next_move(&mut self, c: Color) -> Option<Outcome> {
        let last = self.logic.last_applied_move().map(|m| m.action);
        let action = match c {
            Color::Red => self.red.action_for(self.logic.peek(), last),
            Color::Blk => self.blk.action_for(self.logic.peek(), last),
        };
        let mv = Move { action, player: c };
        self.logic.apply(mv)
    }

    fn game_over(&mut self, outcome: &Outcome) {
        self.red.accept_outcome(&outcome);
        self.blk.accept_outcome(&outcome)
    }
}
