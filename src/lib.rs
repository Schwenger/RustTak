#[macro_use]
extern crate lazy_static;

mod actions;
mod simulator;
mod test_util;

pub mod player;
pub mod board;
pub use simulator::Simulator;
