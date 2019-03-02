use crate::actions::Action;
use crate::board::Position;

use crate::board::piece::PieceKind;
use crate::board::Direction;
use regex::Regex;

pub(crate) struct CLIParser {}

pub(crate) type Result<T> = std::result::Result<T, CLIParserError<T>>;

impl CLIParser {
    pub(crate) fn action(s: &str) -> Result<Action> {
        let s = s.trim().to_lowercase();
        let tokens: Vec<&str> = s.split_whitespace().collect();
        if tokens.is_empty() {
            Err(CLIParserError::new("You didn't say anything...", None))
        } else {
            match tokens[0] {
                "place" | "set" => Self::place(&tokens[1..]),
                "move" | "slide" => Self::slide(&tokens[1..]),
                _ => Err(CLIParserError::new("Unknown command word. Try `place`, `slide`, or `move`.", None)),
            }
        }
    }

    fn slide(tokens: &[&str]) -> Result<Action> {
        Self::exact_slide(tokens)
    }

    fn exact_slide(tokens: &[&str]) -> Result<Action> {
        //        lazy_static! {
        //            static ref regex: Regex = Regex::new(
        //                r#"(stone | flat | standing\s*stone | wall | cap\s*stone)\s*(to | at | on)?\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)"#
        //            ).unwrap();
        //        }
        let regex = Regex::new(
            r#"(?:from)?\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)\s*((north|east|west|south)\s*(?:taking|moving)*\s*((\d*\s*,?\s*)*))?"#
        ).unwrap();

        // Capture groups:
        // 0: full match
        // 1: row
        // 2: col
        // 3: direction
        // 4: optional; full match for optional drops
        // 5: optional; list of carries

        regex
            .captures_iter(&tokens.join(""))
            .map(|cap| {
                let row = cap[1].parse::<usize>().unwrap();
                let col = cap[2].parse::<usize>().unwrap();
                let pos = Position::new(row, col);
                let dir = Self::direction(&cap[3]);
                if cap.len() > 4 {
                    let carries = Self::number_list(&cap[5]);
                    Action::Slide(pos, dir, Some(carries))
                } else {
                    Action::Slide(pos, dir, None)
                }
            })
            .next()
            .map(|a| Ok(a))
            .unwrap_or(Err(CLIParserError::new("I don't understand...", None)))
    }

    fn number_list(s: &str) -> Vec<usize> {
        let regex = Regex::new(r#"\d*"#).unwrap();
        regex.find_iter(s).map(|m| m.as_str().parse::<usize>().unwrap()).collect()
    }

    fn direction(s: &str) -> Direction {
        match s {
            "north" => Direction::North,
            "south" => Direction::South,
            "east" => Direction::East,
            "west" => Direction::West,
            _ => unreachable!(),
        }
    }

    fn place(tokens: &[&str]) -> Result<Action> {
        if let Some(action) = Self::exact_place(tokens) {
            Ok(action)
        } else {
            unimplemented!()
        }
    }

    fn exact_place(tokens: &[&str]) -> Option<Action> {
        lazy_static! {
            static ref regex: Regex = Regex::new(
                r#"(stone | flat | standing\s*stone | wall | cap\s*stone)\s*(to | at | on)?\s*\(\s*(\d+)\s*,\s*(\d+)\s*\)"#
            ).unwrap();
        }
        regex
            .captures_iter(&tokens.join(""))
            .map(|cap| {
                // cap[0] is the full match.
                let kind = Self::piece_kind(&cap[1]);
                // cap[2] is the optional connective.
                let row = cap[3].parse::<usize>().unwrap();
                let col = cap[4].parse::<usize>().unwrap();
                let pos = Position::new(row, col);
                Action::Place(pos, kind)
            })
            .next()
    }

    fn piece_kind(s: &str) -> PieceKind {
        match s {
            "wall" => PieceKind::StandingStone,
            "stone" | "flat" => PieceKind::Stone,
            s if s.starts_with("cap") => PieceKind::CapStone,
            s if s.starts_with("standing") => PieceKind::StandingStone,
            _ => unreachable!(),
        }
    }

    pub(crate) fn position(s: &str) -> Result<Position> {
        if let Some(pos) = Self::exact_position(s) {
            return Ok(pos);
        }
        let s = s.trim().to_lowercase();

        let numbers = Self::extract_numbers(&s);
        if numbers.len() == 2 {
            let best_guess = Some(Position::new(numbers[0], numbers[1]));
            Err(CLIParserError::new("I'm not sure I understood that correctly.", best_guess))
        } else {
            Err(CLIParserError::new("I have no idea what you're talking about.", None))
        }
    }

    fn exact_position(s: &str) -> Option<Position> {
        lazy_static! {
            static ref regex: Regex = Regex::new(r#"\(\s*\d+\s*,\s*\d+\s*\)"#).expect("Meh");
        }
        regex
            .captures_iter(s)
            .map(|cap| Position::new(cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap()))
            .next()
    }

    fn extract_numbers(s: &str) -> Vec<usize> {
        lazy_static! {
            static ref regex: Regex = Regex::new(r#"\d*"#).unwrap();
        }
        regex
            .find_iter(s)
            .map(|m| m.as_str().parse::<usize>().expect("This transformation should always work."))
            .collect()
    }
}

pub(crate) struct CLIParserError<T> {
    pub(crate) help: String,
    pub(crate) best_guess: Option<T>,
}

impl<T> CLIParserError<T> {
    fn new(help: &str, best_guess: Option<T>) -> CLIParserError<T> {
        CLIParserError { help: String::from(help), best_guess }
    }
}
