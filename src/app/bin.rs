use tak_simulator;
use tak_simulator::player::HumanPlayer;

fn main() {
    println!("Nice.");
    let red = HumanPlayer::command_line_interface();
    let blk = HumanPlayer::command_line_interface();
    let sim = tak_simulator::Simulator::new(red, blk, 4);
    sim.start();
}
