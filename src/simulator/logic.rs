use super::game_over::{MatchResult, Outcome};
use crate::actions::{Action, Move};
use crate::board::piece::{Piece, PieceKind};
use crate::board::{Board, Direction, Position};
use crate::player::Color;

use std::collections::HashSet;

pub(crate) struct Logic {
    board: Board,
    red_pieces: PiecesStash,
    blk_pieces: PiecesStash,
    board_size: usize,
    last_applied_move: Option<Move>,
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
            n => ((n as f32).powf(1.88) as u16, (n / 4) as u16), // The first rule seems about right, the second one not so much.
        };
        PiecesStash { stones, caps }
    }
}

enum LogicError {
    NotApplicable(Move),
}

impl Logic {
    pub(crate) fn new(size: usize) -> Logic {
        let stash = PiecesStash::for_board_size(size);
        Logic {
            board: Board::new(size),
            red_pieces: stash,
            blk_pieces: stash,
            board_size: size,
            last_applied_move: None,
        }
    }

    /// Create an ActionLogic for a given board.
    /// Note it is assumed that the last applied move is `None` even if there is only one
    /// logical choice.
    pub(crate) fn from_board(board: Board) -> Logic {
        let mut red_stash = PiecesStash::for_board_size(board.size());
        let mut blk_stash = PiecesStash::for_board_size(board.size());
        use crate::board::piece::PieceKind;
        for stack in board.iter() {
            for piece in stack.iter() {
                match (piece.kind, piece.color) {
                    (PieceKind::CapStone, Color::Red) => red_stash.caps -= 1,
                    (PieceKind::StandingStone, Color::Red) | (PieceKind::Stone, Color::Red) => red_stash.stones -= 1,
                    (PieceKind::CapStone, Color::Blk) => blk_stash.caps -= 1,
                    (PieceKind::StandingStone, Color::Blk) | (PieceKind::Stone, Color::Blk) => blk_stash.stones -= 1,
                }
            }
        }
        let size = board.size();
        Logic { board, red_pieces: red_stash, blk_pieces: blk_stash, board_size: size, last_applied_move: None }
    }

    pub(crate) fn peek(&self) -> &Board {
        &self.board
    }

    pub(crate) fn first_turn(&mut self, pos: Position, c: Color) {
        self.apply(Move { player: !c, action: Action::Place(pos, PieceKind::Stone) });
    }

    pub(crate) fn last_applied_move(&mut self) -> Option<Move> {
        self.last_applied_move.clone()
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
                let too_many = || self.board_size < v.iter().sum();
                let color = || self.board[pos].color().unwrap() == mv.player;
                let oob = || match dir {
                    Direction::North => pos.row + v.len() > size,
                    Direction::East => pos.col + v.len() > size,
                    Direction::South => pos.row < v.len(),
                    Direction::West => pos.col < v.len(),
                };
                let all_compatible = || {
                    let mut src = pos;
                    let original_stack = &self.board[src];
                    if original_stack.len() < *v.first().unwrap() {
                        return false;
                    }
                    if original_stack.color().map(|c| c != mv.player).unwrap_or(false) {
                        return false; // If there is a stack, it's color must be the player's.
                    }
                    let mut carried = original_stack.peek_from_top(*v.first().unwrap());
                    for n in &v[1..] {
                        let dst = src.go(dir);
                        if !self.board[dst].compatible_with(&carried) {
                            return false;
                        }
                        src = dst;
                        carried = carried.peek_from_top(*n);
                    }
                    true
                };
                dbg!(val_pos())
                    && dbg!(!empty())
                    && dbg!(!contains_0())
                    && dbg!(!too_many())
                    && dbg!(color())
                    && dbg!(!oob())
                    && dbg!(all_compatible())
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
                    let new_top = self.board[src].take_off(n);
                    self.board[dst] += new_top;
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
                Direction::East => pos.col == self.board_size - 1,
                Direction::West => pos.col == 0,
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
        let mut frontier: HashSet<Position> =
            (0..self.board_size).map(|ix| vec![Position::new(0, ix), Position::new(ix, 0)]).flatten().collect();

        while !frontier.is_empty() {
            frontier = frontier
                .drain()
                .flat_map(collect_neighbours)
                .filter(|p| {
                    let stack = &self.board[*p];
                    stack.is_road() && stack.color().map(|k| k == c).unwrap_or(false)
                })
                .collect();
            if frontier.iter().any(is_goal) {
                return true;
            }
            frontier.difference(&closed);
            closed.extend(frontier.iter());
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::board::piece::Stack;
    use crate::player::Color::*;
    use crate::test_util::*;

    fn parse_single(s: &str) -> Stack {
        let s = s.to_lowercase();
        if s.len() == 1 {
            assert_eq!(s, "!");
            Stack::empty()
        } else {
            assert_eq!(s.len() % 2, 0);
            let mut stack = Vec::new();
            let mut cs = s.chars();
            while let Some(c) = cs.next() {
                let color = match c {
                    'r' => Red,
                    'b' => Blk,
                    sym => panic!("Unrecognized symbol `{}`.", sym),
                };
                let kind = match cs.next() {
                    Some('s') => PieceKind::Stone,
                    Some('w') | Some('x') => PieceKind::StandingStone,
                    Some('c') => PieceKind::CapStone,
                    Some(_) => panic!("Unrecognized symbol"),
                    None => unreachable!(),
                };
                stack.push(Piece::new(kind, color));
            }
            Stack::from(stack)
        }
    }

    fn parse(width: usize, s: &str) -> Board {
        let mut row = 0;
        let mut board = Board::new(width);
        for (ix, stack) in s.split_whitespace().map(parse_single).enumerate() {
            if ix == width {
                row += 1;
            }
            let col = ix % width;
            board[Position::new(row, col)] = stack;
        }
        board
    }

    fn apply(b: &str, width: usize, action: Action, color: Color) -> (Logic, Option<Outcome>) {
        let board = parse(width, b);
        let mut ml = Logic::from_board(board);
        let oc = ml.apply(Move { action, player: color });
        (ml, oc)
    }

    fn applicable(b: &str, width: usize, action: Action, color: Color) -> bool {
        let board = parse(width, b);
        let ml = Logic::from_board(board);
        ml.applicable(&Move { action, player: color })
    }

    #[test]
    fn test_place_on_empty() {
        let s = "\
        ! ! !
        ! ! !
        ! ! !
        ";
        let target = Position::new(1, 2);
        let action = Action::Place(target, PieceKind::Stone);
        let (ml, oc) = apply(s, 3, action, Red);
        let board = ml.peek();
        for (pos, stack) in board.iter().with_pos() {
            if pos == target {
                assert_eq!(stack, &single_stone(Red));
            } else {
                assert_eq!(stack, &Stack::empty());
            }
        }
        //        assert!(oc.is_none())
    }

    #[test]
    fn test_applicable_place_invalid() {
        let s = "\
        RS ! !
        !  ! !
        !  ! !
        ";
        let target = Position::new(0, 0);
        let action = Action::Place(target, PieceKind::Stone);
        assert!(!applicable(s, 3, action, Red));
    }

    #[test]
    fn test_applicable_place_invalid_flatten() {
        let s = "\
        RX ! !
        !  ! !
        !  ! !
        ";
        let target = Position::new(0, 0);
        let action = Action::Place(target, PieceKind::CapStone);
        assert!(!applicable(s, 3, action, Red));
    }

    #[test]
    fn test_applicable_invalid_move_on_wall() {
        let s = "\
        RX RS !
        !  ! !
        !  ! !
        ";
        let source = Position::new(0, 1);
        let action = Action::Slide(source, Direction::West, vec![1]);
        assert!(!applicable(s, 3, action, Red));
    }

    #[test]
    fn test_apply_move_on_stone() {
        let start = "\
        RS RS !
        !  ! !
        !  ! !
        ";
        let expected = parse(
            3,
            "\
        RSRS ! !
        !    ! !
        !    ! !
        ",
        );
        let source = Position::new(0, 1);
        let action = Action::Slide(source, Direction::West, vec![1]);
        let (ml, oc) = apply(start, 3, action, Red);
        //        assert!(oc.is_none());
        let was = ml.peek();
        assert_eq!(was, &expected);
    }
}
