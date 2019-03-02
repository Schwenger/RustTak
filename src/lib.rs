#[macro_use]
extern crate lazy_static;

mod actions;
mod board;
mod simulator;
mod test_util;

pub mod player;
pub use simulator::Simulator;
