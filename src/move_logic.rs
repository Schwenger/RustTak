
use crate::board::{Board, Position, Direction};
use crate::actions::{Move, Action};
use crate::player::Color;
use crate::simulator::{Outcome, MatchResult};
use crate::board::piece::{Piece, PieceKind};

use std::collections::HashSet;

pub(crate) struct MoveLogic {
    board: Board,
    red_pieces: PiecesStash,
    blk_pieces: PiecesStash,
    board_size: usize,
    pub(crate) last_applied_move: Option<Move>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PiecesStash {
    stones: u16,
    caps: u16,
}

impl PiecesStash {
    fn for_board_size(n: usize) -> PiecesStash {
        let (stones, caps) = match n {
            3 => (10, 0),
            4 => (15, 0),
            5 => (21, 1),
            6 => (30, 1),
            8 => (50, 2),
            _ => panic!("Board size {} is not supported.") // TODO: Make sure this does not happen.
            // TODO: Maybe even translate sizes into enum with all required constants contained.
        };
        PiecesStash { stones, caps }
    }
}

impl MoveLogic {

    pub(crate) fn new(size: usize) -> MoveLogic {
        let stash = PiecesStash::for_board_size(size);
        MoveLogic {
            board: Board::new(size),
            red_pieces: stash,
            blk_pieces: stash,
            board_size: size,
            last_applied_move: None,
        }
    }

    pub(crate) fn peek(&self) -> &Board {
        &self.board
    }

    pub(crate) fn first_turn(&mut self, pos: Position, c: Color) {
        self.apply(Move { player: !c, action: Action::Place(pos, PieceKind::Stone) });
    }

    fn valid_pos(&self, pos: Position) -> bool {
        self.board.valid_pos(pos)
    }

    fn piece_count_mut(&mut self, c: Color, kind: PieceKind) -> &mut u16 {
        match (c, kind) {
            (Color::Blk, PieceKind::CapStone) => &mut self.blk_pieces.caps,
            (Color::Red, PieceKind::CapStone) => &mut self.red_pieces.caps,
            (Color::Blk, _) => &mut self.blk_pieces.stones,
            (Color::Red, _) => &mut self.red_pieces.stones,
        }
    }

    fn piece_count(&self, c: Color, kind: PieceKind) -> u16 {
        match (c, kind) {
            (Color::Blk, PieceKind::CapStone) => self.blk_pieces.caps,
            (Color::Red, PieceKind::CapStone) => self.red_pieces.caps,
            (Color::Blk, _) => self.blk_pieces.stones,
            (Color::Red, _) => self.red_pieces.stones,
        }
    }

    fn take_piece(&mut self, c: Color, kind: PieceKind) -> Piece {
        let n = self.piece_count_mut(c, kind);
        *n -= 1;
        Piece::new(kind, c)
    }

    // TODO: Return error message.
    pub(crate) fn applicable(&self, mv: &Move) -> bool {
        match mv.action {
            Action::Place(pos, kind) => {
                self.valid_pos(pos) && self.piece_count(mv.player, kind) > 0 && self.board[pos].is_empty()
            }
            Action::Slide(pos, dir, ref v) => {
                let size = self.board_size; // Abbreviation
                let val_pos = || self.valid_pos(pos);
                let empty = || v.is_empty();
                let contains_0 = || v.iter().find(|n| **n == 0).is_some();
                let too_many = || self.board_size >= v.iter().sum();
                let color = || self.board[pos].color().unwrap() == mv.player;
                let oob = || {
                    match dir {
                        Direction::North => pos.row + v.len() > size,
                        Direction::East => pos.col + v.len() > size,
                        Direction::South => pos.row < v.len(),
                        Direction::West => pos.col < v.len(),
                    }
                };
                let all_comp = || {
                    let mut src = pos;
                    let mut carried = self.board[src].peek_from_top(*v.first().unwrap());
                    for n in &v[1..] {
                        let dst = src.go(dir);
                        if !self.board[dst].compatible_with(&carried) {
                            return false
                        }
                        src = dst;
                        carried = carried.peek_from_top(*n);
                    }
                    true
                };
                val_pos() && !empty() && !contains_0() && !too_many() && color() && !oob() && all_comp()
            }
        }
    }

    pub(crate) fn apply(&mut self, mv: Move) -> Option<Outcome> {
        debug_assert!(self.applicable(&mv));
        match mv.action {
            Action::Place(pos, kind) => {
                let piece = self.take_piece(mv.player, kind);
                self.board[pos] += piece;
            }
            Action::Slide(pos, dir, v) => {
                let mut src = pos;
                for n in v {
                    let dst = src.go(dir);
                    self.board[dst] += self.board[src].take_off(n);
                    src = dst;
                }
            }
        }
        self.get_outcome()
    }

    fn get_outcome(&self) -> Option<Outcome> {
        // Naive approach:
        let res = match (self.is_winner(Color::Red), self.is_winner(Color::Blk)) {
            (true, true) => MatchResult::Tie,
            (true, false) => MatchResult::Winner(Color::Red),
            (false, true) => MatchResult::Winner(Color::Blk),
            (false, false) => return None,
        };
        Some(Outcome { result: res, board: self.board.clone() })

    }

    fn is_winner(&self, c: Color) -> bool {
        self.is_winner_for_dir(c, Direction::North) || self.is_winner_for_dir(c, Direction::East)
    }

    fn is_winner_for_dir(&self, c: Color, dir: Direction) -> bool {
        // Helper functions:
        let is_goal = |pos: &Position| -> bool {
            match dir {
                Direction::North => pos.row == self.board_size - 1,
                Direction::South => pos.row == 0,
                Direction::East  => pos.col == self.board_size - 1,
                Direction::West  => pos.col == 0,
            }
        };
        let collect_neighbours = |pos: Position| -> Vec<Position> {
            use crate::board::Direction::*;
            let mut res = Vec::new();
            if pos.col > 0 {
                res.push(pos.go(West))
            }
            if pos.col < self.board_size - 1 {
                res.push(pos.go(East))
            }
            if pos.row > 0 {
                res.push(pos.go(South))
            }
            if pos.row < self.board_size - 1 {
                res.push(pos.go(North))
            }
            res
        };

        // Naive approach:
        let mut closed = HashSet::new();
        let mut frontier: HashSet<Position> = (0..self.board_size)
            .map(|ix| vec![Position::new(0, ix), Position::new(ix, 0)])
            .flatten()
            .collect();

        while !frontier.is_empty() {
            frontier = frontier.drain().flat_map(collect_neighbours).collect();
            if frontier.iter().any(is_goal) {
                return true
            }
            frontier.difference(&closed);
            closed.extend(frontier.iter());
        }
        false
    }

}
