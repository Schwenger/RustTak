
use crate::actions::Action;
use crate::board::Position;
use std::error::Error;
use std::fmt::{Debug, Display, Result as FmtResult, Formatter};

pub(crate) struct CLIParser {}

pub(crate) type Result<T> = std::result::Result<T, CLIParserError<T>>;

impl CLIParser {
    pub(crate) fn action(s: &String) -> Result<Action> {
        unimplemented!()
    }
    pub(crate) fn position(s: &String) -> Result<Position> {
        unimplemented!()
    }
}

pub(crate) struct CLIParserError<T> {
    pub(crate) help: String,
    pub(crate) best_guess: Option<T>,
}
