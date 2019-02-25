use crate::player::command_line_human::CommandLineHuman;
use crate::player::Color;
use crate::player::Player;
use crate::simulator::Simulator;

mod actions;
mod board;
mod move_logic;
mod player;
mod simulator;
mod test_util;

pub fn start() {
    let red = CommandLineHuman::setup(4, Color::Red, true);
    let blk = CommandLineHuman::setup(4, Color::Blk, false);
    let sim = Simulator::new(red, blk, 4);
    sim.start();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
