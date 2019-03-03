#[macro_use]
extern crate lazy_static;

mod actions;
mod simulator;
mod test_util;

pub mod analyzer;
pub mod board;
pub mod player;
pub use simulator::Simulator;
