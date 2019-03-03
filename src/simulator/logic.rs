use super::game_over::{MatchResult, Outcome};
use crate::actions::{Action, Move};
use crate::board::piece::{Piece, PieceKind};
use crate::board::{Board, Direction, Position};
use crate::player::Color;

use std::collections::HashSet;

pub(crate) struct Logic {
    board: Board,
    last_applied_move: Option<Move>,
}

impl Logic {
    pub(crate) fn new(size: usize) -> Logic {
        Logic { board: Board::new(size), last_applied_move: None }
    }

    /// Create an ActionLogic for a given board.
    /// Note it is assumed that the last applied move is `None` even if there is only one
    /// logical choice.
    #[cfg(test)]
    fn from_board(board: Board) -> Logic {
        Logic { board, last_applied_move: None }
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

    fn size(&self) -> usize {
        self.board.size()
    }

    fn valid_pos(&self, pos: Position) -> bool {
        self.board.valid_pos(pos)
    }

    pub(crate) fn applicable(&self, mv: &Move) -> bool {
        match mv.action {
            Action::Place(pos, kind) => {
                self.valid_pos(pos) && self.board[pos].is_empty() && self.board.piece_count(mv.player, kind) > 0
            }
            Action::Slide(pos, dir, ref v) => {
                let size = self.size(); // Abbreviation
                let v = v.as_ref().map(Vec::clone).unwrap_or_else(|| vec![self.board[pos].len()]);

                let val_pos = || self.valid_pos(pos);
                let empty = || v.is_empty();
                let contains_0 = || v.iter().any(|n| *n == 0);
                let decreasing = || {
                    let mut last = v.first().unwrap();
                    for current in &v[1..] {
                        if current >= last {
                            return false;
                        }
                        last = current;
                    }
                    true
                };
                let color = || self.board[pos].color().map(|c| c == mv.player).unwrap_or(true);
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
                    let mut full = original_stack.clone();
                    for n in &v {
                        let dst = src.go(dir);
                        let carried = full.peek_from_top(*n);
                        if !self.board[dst].compatible_with(&carried) {
                            return false;
                        }
                        src = dst;
                        full = carried;
                    }
                    true
                };
                val_pos() && !empty() && !contains_0() && decreasing() && color() && !oob() && all_compatible()
            }
        }
    }

    pub(crate) fn apply(&mut self, mv: Move) -> Option<Outcome> {
        debug_assert!(self.applicable(&mv));
        match mv.action {
            Action::Place(pos, kind) => {
                self.board.place(Piece::new(kind, mv.player), pos);
            }
            Action::Slide(pos, dir, v) => {
                let mut src = pos;
                let v = v.unwrap_or_else(|| vec![self.board[pos].len()]);
                for n in v {
                    self.board.slide(src, dir, n);
                    src = src.go(dir);
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
                Direction::North => pos.row == self.size() - 1,
                Direction::South => pos.row == 0,
                Direction::East => pos.col == self.size() - 1,
                Direction::West => pos.col == 0,
            }
        };
        let collect_neighbours = |pos: Position| -> Vec<Position> {
            use crate::board::Direction::*;
            let mut res = Vec::new();
            if pos.col > 0 {
                res.push(pos.go(West))
            }
            if pos.col < self.size() - 1 {
                res.push(pos.go(East))
            }
            if pos.row > 0 {
                res.push(pos.go(South))
            }
            if pos.row < self.size() - 1 {
                res.push(pos.go(North))
            }
            res
        };
        let qualifies = |p: &Position| {
            let stack = &self.board[*p];
            stack.is_road() && stack.color().map(|k| k == c).unwrap_or(false)
        };

        // Naive approach:
        let mut closed = HashSet::new();
        let mut frontier: HashSet<Position> = (0..self.size())
            .map(|ix| match dir {
                Direction::North => Position::new(0, ix),
                Direction::South => Position::new(self.board.size() - 1, ix),
                Direction::East => Position::new(ix, 0),
                Direction::West => Position::new(ix, self.board.size() - 1),
            })
            .filter(qualifies)
            .collect();

        while !frontier.is_empty() {
            frontier = frontier.difference(&closed).cloned().collect();
            closed.extend(frontier.iter());
            frontier = frontier.drain().flat_map(collect_neighbours).filter(qualifies).collect();
            if frontier.iter().any(is_goal) {
                return true;
            }
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
        let mut row = width; // Row 0 is supposed to be the last row.
        let mut col = 0;
        let mut board = Board::new(width);
        for stack in s.split_whitespace().map(parse_single) {
            if !stack.is_empty() {
                board.set_forcefully(Position::new(row - 1, col), stack);
            }
            col += 1;
            if col == width {
                col = 0;
                row -= 1;
            }
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
        assert!(oc.is_none())
    }

    #[test]
    fn test_applicable_place_invalid() {
        let s = "\
        RS ! !
        !  ! !
        !  ! !
        ";
        let target = Position::new(2, 0);
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
        let source = Position::new(2, 1);
        let action = Action::Slide(source, Direction::West, Some(vec![1]));
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
        let source = Position::new(2, 1);
        let action = Action::Slide(source, Direction::West, Some(vec![1]));
        let (ml, oc) = apply(start, 3, action, Red);
        assert!(oc.is_none());
        let was = ml.peek();
        assert_eq!(was, &expected);
    }

    #[test]
    fn test_apply_move_on_wall() {
        let start = "\
        RX RC ! ! !
        !  !  ! ! !
        !  !  ! ! !
        !  !  ! ! !
        ";
        let expected = parse(
            5,
            "\
        RSRC ! ! ! !
        !    ! ! ! !
        !    ! ! ! !
        !    ! ! ! !
        ",
        );
        let source = Position::new(4, 1);
        let action = Action::Slide(source, Direction::West, Some(vec![1]));
        let (ml, oc) = apply(start, 5, action, Red);
        assert!(oc.is_none());
        let was = ml.peek();
        assert_eq!(was, &expected);
    }

    #[test]
    fn test_apply_slide() {
        let start = "\
        !        !  ! !
        !        !  ! !
        RSBSRSRX RS ! !
        !        !  ! !
        ";
        let expected = parse(
            4,
            "\
        !  !    !    !
        !  !    !    !
        RS RSBS RSRX !
        !  !    !    !
        ",
        );
        let source = Position::new(1, 0);
        let action = Action::Slide(source, Direction::East, Some(vec![3, 2]));
        let (ml, oc) = apply(start, 4, action, Red);
        assert!(oc.is_none());
        let was = ml.peek();
        assert_eq!(was, &expected);
    }

    #[test]
    fn test_apply_slide_invalid_non_dec() {
        let start = "\
        !        !  ! !
        !        !  ! !
        RSBSRSRX RS ! !
        !        !  ! !
        ";
        let source = Position::new(1, 0);
        let action = Action::Slide(source, Direction::East, Some(vec![3, 3]));
        assert!(!applicable(start, 4, action, Red));
    }

    #[test]
    fn test_apply_slide_invalid_too_many() {
        let start = "\
        !        !  ! !
        !        !  ! !
        RSBSRSRX RS ! !
        !        !  ! !
        ";
        let source = Position::new(1, 0);
        let action = Action::Slide(source, Direction::East, Some(vec![5, 1]));
        assert!(!applicable(start, 4, action, Red));
    }

    #[test]
    fn test_game_over() {
        let start = "\
        !          !  ! ! !
        !          !  ! ! !
        !          !  ! ! !
        RSRSRSRSRC BS ! ! !
        !          !  ! ! !
        ";
        let expected = parse(
            5,
            "\
        !  !    !  !  !
        !  !    !  !  !
        !  !    !  !  !
        RS BSRS RS RS RC
        !  !    !  !  !
        ",
        );
        let source = Position::new(1, 0);
        let action = Action::Slide(source, Direction::East, Some(vec![4, 3, 2, 1]));
        let (ml, oc) = apply(start, 5, action, Red);
        let was = ml.peek();
        assert_eq!(was, &expected);
        assert!(oc.is_some());
        let oc = oc.unwrap();
        assert_eq!(oc.board, expected);
        assert_eq!(oc.result, MatchResult::Winner(Color::Red));
    }
}
