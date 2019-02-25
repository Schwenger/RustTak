
use super::{Player, Outcome};
use crate::board::{Board, Position};
use crate::actions::{Move, Action};
use std::io::stdin;

mod printer;
mod cli_parser;
use self::printer::{CLHumanDisplay, CIBoardPrinter};
use self::cli_parser::CLIParser;

#[derive(Debug, Clone)]
struct CommandLineHuman {
    name: String,
    printer: CIBoardPrinter,
}

impl CommandLineHuman {
    fn new(board_size: usize) -> CommandLineHuman {
        println!("Welcome to Tak!\nWhat's your name?");
        let mut name = String::new();
        let _ = stdin().read_line(&mut name);
        let printer = CIBoardPrinter::new(board_size);
        CommandLineHuman { name, printer }
    }
}

impl Player for CommandLineHuman {

    fn welcome(&self, first: bool, opponent: &String) {
        println!("{}, you'll be facing {} today.", self.name, opponent);
        println!("You will go {}.", if first { "first" } else { "second" });
    }

    fn action_for(&self, board: &Board, opponent_action: Option<Action>) -> Action {
        if let Some(action) = opponent_action {
            println!("The opponent {}.", action.cl_display());
            println!("Now, the situation is as follows:");
        } else {
            println!("The situation is as follows:");
        }
        println!("{}", self.printer.print(board));
        println!("What do you want to do? (Place/Slide/Move)");
        let mut command = String::new();
        stdin().read_line(&mut command);
        CLIParser::action(&command)
    }

    fn first_action(&self, board: &Board) -> Position {
        unimplemented!()
    }

    fn accept_outcome(&self, outcome: &Outcome) {
        unimplemented!()
    }

    fn name(&self) -> &String {
        &self.name
    }
}

