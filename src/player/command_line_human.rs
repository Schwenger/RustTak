use super::{Color, Player};
use crate::actions::Action;
use crate::board::{Board, Position};
use crate::simulator::game_over::{MatchResult, Outcome};
use std::io::stdin;

mod cli_parser;
mod printer;
use self::cli_parser::{CLIParser, CLIParserError};
use self::printer::{CIBoardPrinter, CLHumanDisplay};

#[derive(Debug, Clone)]
pub struct CommandLineHuman {
    name: String,
    printer: CIBoardPrinter,
    color: Color,
    opponent: String,
    first: bool,
}

impl CommandLineHuman {
    pub(crate) fn new(name: String, board_size: usize, color: Color, first: bool) -> CommandLineHuman {
        let printer = CIBoardPrinter::new(board_size);
        CommandLineHuman { name, printer, color, first, opponent: String::from("Karen") }
    }

    fn read_yes_no() -> bool {
        let mut response = String::new();
        let _ = stdin().read_line(&mut response);
        match response.to_lowercase().as_str() {
            "yes" | "y" | "si" | "ja" | "yeah" | "yea" | "sure" | "ofc" | "what else?" => true,
            "no" | "n" | "nope" | "nein" | "nah" | "nay" | "not at all" | "niet" | "u mad?" => false,
            _ => {
                println!("Sorry, I failed to map {} to yes or no. I'll just take it as a no.", response);
                false
            }
        }
    }

    fn ask_in_loop<T: CLHumanDisplay, F>(&self, f: F) -> T
    where
        F: Fn(&String) -> Result<T, CLIParserError<T>>,
    {
        loop {
            let mut response = String::new();
            let _ = stdin().read_line(&mut response);
            match f(&response) {
                Ok(value) => return value,
                Err(e) => {
                    println!("{}", e.help);
                    if let Some(guess) = e.best_guess {
                        println!("Did you mean: {}", guess.cl_display());
                        if Self::read_yes_no() {
                            return guess;
                        }
                    }
                    println!("Go ahead, try again! :)");
                }
            }
        }
    }
}

impl Player for CommandLineHuman {
    fn welcome(&mut self, opponent: &String) {
        println!("{}, you'll be facing {} today.", self.name, opponent);
        println!("You will go {}.", if self.first { "first" } else { "second" });
        self.opponent = opponent.clone();
    }

    fn action_for(&mut self, board: &Board, opponent_action: Option<Action>) -> Action {
        if let Some(action) = opponent_action {
            println!("The opponent {}.", action.cl_display());
            println!("Now, the situation is as follows:");
        } else {
            println!("The situation is as follows:");
        }
        println!("{}", self.printer.print(board));
        println!("What do you want to do? (Place/Slide/Move)");
        let mut command = String::new();
        let _ = stdin().read_line(&mut command);
        self.ask_in_loop(CLIParser::action)
    }

    fn first_action(&mut self, board: &Board) -> Position {
        if !self.first {
            println!(
                "After {}'s first turn, the board looks as follows: \n{}",
                self.opponent,
                self.printer.print(board)
            );
        }
        println!("Let's get started. Where do you want to place {}'s first piece?", self.opponent);
        println!("You can enter positions using cartesiean notation, i.e., (1,3) for row 1, column 3, counting bottom to top, left to right.");
        self.ask_in_loop(CLIParser::position)
    }

    fn accept_outcome(&mut self, outcome: &Outcome) {
        match outcome.result {
            MatchResult::Winner(c) if c == self.color => println!("Congratulations, you won, {}!", self.name),
            MatchResult::Winner(_) => println!("Congratulations, you suck! {} beat you with ease.", self.opponent),
            MatchResult::Tie => println!("Congratulations, you're just as bad as {}.", self.opponent),
        }
        println!("The final board looked as follows: \n{}", self.printer.print(&outcome.board));
    }

    fn name(&self) -> &String {
        &self.name
    }
}
