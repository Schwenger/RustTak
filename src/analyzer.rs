use crate::actions::Action;
use crate::board::piece::PieceKind;
use crate::board::piece::Stack;
use crate::board::Board;
use crate::board::Direction;
use crate::board::Position;
use crate::player::Color;
use std::cmp::{max, min};

pub struct Analyzer<'a> {
    board: &'a Board,
}

pub struct Metric<T: Default + Copy> {
    red: T,
    blk: T,
}

impl<T: Default + Copy> Metric<T> {
    pub(crate) fn new() -> Metric<T> {
        Metric { red: T::default(), blk: T::default() }
    }

    pub(crate) fn for_values(red: T, blk: T) -> Metric<T> {
        Metric { red, blk }
    }

    pub fn of(&self, player: Color) -> T {
        match player {
            Color::Red => self.red,
            Color::Blk => self.blk,
        }
    }

    pub fn of_mut(&mut self, player: Color) -> &mut T {
        match player {
            Color::Red => &mut self.red,
            Color::Blk => &mut self.blk,
        }
    }
}

impl<'a> Analyzer<'a> {
    pub fn for_board(board: &'a Board) -> Analyzer<'a> {
        Analyzer { board }
    }

    pub fn longest_road(&self) -> (u16, u16) {
        // Prepare function in MoveLogic for this.
        unimplemented!()
    }

    pub fn absolute_road_dominance(&self) -> Metric<u16> {
        self.board.iter().flat_map(|s| s.color()).fold(Metric::new(), |mut metric, c| {
            *metric.of_mut(c) += 1;
            metric
        })
    }

    pub fn stones_left(&self) -> Metric<u16> {
        let red = self.board.piece_count(Color::Red, PieceKind::Stone);
        let blk = self.board.piece_count(Color::Blk, PieceKind::Stone);
        Metric::for_values(red, blk)
    }

    pub fn caps_left(&self) -> Metric<u16> {
        let red = self.board.piece_count(Color::Red, PieceKind::CapStone);
        let blk = self.board.piece_count(Color::Blk, PieceKind::CapStone);
        Metric::for_values(red, blk)
    }

    pub fn highest_stack(&self) -> Metric<usize> {
        self.board.iter().filter(|s| s.is_empty()).fold(Metric::new(), |mut m, s| {
            let v = m.of_mut(s.color().unwrap());
            *v = max(*v, s.len());
            m
        })
    }

    pub fn applicable_actions(&self, player: Color) -> Vec<Action> {
        type Fields<'a> = Vec<(Position, &'a Stack)>;
        let (free, occupied): (Fields, Fields) = self.board.iter().with_pos().partition(|(_, stack)| stack.is_empty());
        let free: Vec<Position> = free.into_iter().map(|(pos, _)| pos).collect();

        let mut actions = Vec::new();

        if self.board.piece_count(player, PieceKind::Stone) > 0 {
            actions.extend(free.iter().map(|pos| Action::Place(*pos, PieceKind::Stone)));
            actions.extend(free.iter().map(|pos| Action::Place(*pos, PieceKind::StandingStone)));
        }
        if self.board.piece_count(player, PieceKind::CapStone) > 0 {
            actions.extend(free.iter().map(|pos| Action::Place(*pos, PieceKind::CapStone)));
        }

        fn make_slides(src: Position, dir: Direction, stack: &Stack, size: usize) -> Vec<Action> {
            let space_to_border = match dir {
                Direction::North => size - 1 - src.row,
                Direction::South => src.row,
                Direction::East => size - 1 - src.col,
                Direction::West => src.col,
            };
            fn distribute(pieces: usize, parts: usize) -> Vec<usize> {
                if parts == 1 {
                    vec![pieces]
                } else {
                    (1..=(pieces - parts))
                        .flat_map(|taken| {
                            let mut res = vec![taken];
                            res.extend(distribute(pieces - taken, parts - 1));
                            res
                        })
                        .collect()
                }
            }
            let effective_size = max(stack.len(), size);
            (1..effective_size)
                .flat_map(|pieces| {
                    (1..min(effective_size, space_to_border)).map(move |parts| distribute(pieces, parts))
                })
                .map(|v| Action::Slide(src, dir, Some(v)))
                .collect()
        }

        let slides = occupied
            .into_iter()
            .filter(|(_, stack)| stack.color().unwrap() == player) // Unwrap safe, only occupied fields.
            .flat_map(|(pos, stack)| {
                vec![
                    make_slides(pos, Direction::North, stack, self.board.size()),
                    make_slides(pos, Direction::South, stack, self.board.size()),
                    make_slides(pos, Direction::East, stack, self.board.size()),
                    make_slides(pos, Direction::West, stack, self.board.size()),
                ].into_iter().flatten()
            });

        actions.extend(slides);
        actions
    }
}
