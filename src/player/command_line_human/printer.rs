use crate::actions::Action;
use crate::board::piece::{Stack, Piece, PieceKind};
use crate::board::{Position, Board, Direction};
use crate::player::Color;
use colored::*;
use std::cell::RefCell;

pub(crate) trait CLHumanDisplay {
    fn cl_display(&self) -> ColoredString;
}

impl CLHumanDisplay for Action {
    fn cl_display(&self) -> ColoredString {
        match self {
            Action::Place(pos, kind) => format!("placed a {} at {}", pos.cl_display(), kind.cl_display()).normal(),
            Action::Slide(src, dir, v) if v.len() == 1 =>
                format!("moved {} pieces from {} {}", v[0], src.cl_display(), dir.cl_display()).normal(),
            Action::Slide(src, dir, v) => {
                let takes = v.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(", ");
                format!("started sliding {} from {} taking {} pieces", dir.cl_display(), src.cl_display(), takes).normal()
            }
        }
    }
}

impl CLHumanDisplay for PieceKind {
    fn cl_display(&self) -> ColoredString {
        match self {
            PieceKind::Stone => ColoredString::from("🝙"),
            PieceKind::StandingStone => ColoredString::from("╳"),
            PieceKind::CapStone => ColoredString::from(/*"♜"*/ "👑"),
        }
    }
}

impl CLHumanDisplay for Position {
    fn cl_display(&self) -> ColoredString {
        let c: u8 = 65u8 + (self.col as u8);
        format!("({}, {})", self.row, c as char).normal()
    }
}

impl CLHumanDisplay for Direction {
    fn cl_display(&self) -> ColoredString {
        match self {
            Direction::North => ColoredString::from("upwards"),
            Direction::South => ColoredString::from("downwards"),
            Direction::East  => ColoredString::from("to the right"),
            Direction::West  => ColoredString::from("to the left"),
        }
    }
}

impl CLHumanDisplay for Piece {
    fn cl_display(&self) -> ColoredString {
        match self.color {
            Color::Red => self.kind.cl_display().red(),
            Color::Blk => self.kind.cl_display().black(),
        }

    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoardCell {
    inner: Vec<Vec<ColoredString>>,
}

// TODO: CELL_WIDTH should depend on board size.
// TODO: Should be odd.
const CELL_WIDTH: usize = 3;
impl BoardCell {
    fn default() -> BoardCell {
        BoardCell { inner: Vec::new() }
    }
    fn empty() -> BoardCell {
        BoardCell { inner: vec![vec![ColoredString::from(" "); CELL_WIDTH]; CELL_WIDTH] }
    }
    fn straight() -> BoardCell {
        BoardCell { inner: vec![vec!["-".bold(); CELL_WIDTH]] }
    }
    fn pipe() -> BoardCell {
        BoardCell { inner: vec![vec!["|".bold()]; CELL_WIDTH] }
    }
    fn cross() -> BoardCell {
        BoardCell { inner: vec![vec!["┼".bold()]] }
    }

    fn from(mut s: &Stack) -> BoardCell {
        let mut s = s.clone(); // TODO: Avoid this clone.
        let num_printable = CELL_WIDTH * CELL_WIDTH;
        let truncate = s.len() > num_printable;
        if truncate {
            s.peek_from_top(num_printable - 1);
            unimplemented!()
        }
        let num_rows = (s.len() as f32 / CELL_WIDTH as f32).ceil() as usize;

        let mut sieve = |_: usize| -> Stack {
            if s.len() > CELL_WIDTH {
                s.take_off(CELL_WIDTH)
            } else {
                s.clone()
            }
        };

        let rows = (0..num_rows).map(sieve).map(Self::transform_single_row_stack).collect();
        let content = Self::distribute_rows(rows);

        BoardCell { inner: content }
    }

    // TODO: Solve string/colored_string conundrum.
    fn get_row(&self, i: usize) -> String {
        assert!(i < self.inner.len());
        self.inner[i].iter().fold(String::new(), |s1, s2| s1 + &s2.to_string())
    }

    fn distribute_rows(rows: Vec<Vec<ColoredString>>) -> Vec<Vec<ColoredString>> {
        assert!(rows.len() < CELL_WIDTH);
        let space_left_over = CELL_WIDTH - rows.len();
        let space_top = space_left_over / 2;
        (0..CELL_WIDTH).map(|ix| {
            if ix > space_top && ix < space_top + rows.len() {
                rows[ix - space_top].clone() // TODO: This clone is avoidable.
            } else {
                vec![ColoredString::from(""); CELL_WIDTH]
            }
        }).collect()
    }

    fn transform_single_row_stack(s: Stack) -> Vec<ColoredString> {
        assert!(s.len() <= CELL_WIDTH);
        let mut row = vec![ColoredString::from(" "); CELL_WIDTH];
        let leftover_space = CELL_WIDTH - s.len();
        let space_right = leftover_space / 2;
//        let mut i = space_right;
        for (i, piece) in s.iter().enumerate() {
            row[i] = ColoredString::from(piece.cl_display());
        }
        row
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CIBoardPrinter {
    board: RefCell<Vec<Vec<BoardCell>>>,
}

impl CIBoardPrinter {
    pub(crate) fn new(board_size: usize) -> CIBoardPrinter {
        let mut board = vec![vec![BoardCell::default(); board_size + 2]; board_size + 2];
        // Draw crosses in corners.
        board[0][0] = BoardCell::cross();
        board[board_size][0] = BoardCell::cross();
        board[0][board_size] = BoardCell::cross();
        board[board_size][board_size] = BoardCell::cross();
        for i in 1..(board_size + 1) {
            // Draw straight borders vertically and then horizontally.
            board[1][i] = BoardCell::pipe();
            board[board_size + 1][i] = BoardCell::pipe();
            board[i][1] = BoardCell::straight();
            board[i][board_size+1] = BoardCell::straight();
            // Any other field is just empty.
            for j in 1..(board_size + 1) {
                board[i][j] = BoardCell::empty();
            }
        }
        CIBoardPrinter { board: RefCell::new(board) }
    }

    pub(crate) fn print(&self, board: &Board) -> String {
        self.translate_board(board);
        let mut accu = String::new();
        let inner = self.board.borrow();
        for row in 0..board.size() {
            for col in 0..board.size() {
                for cell_row in 0..CELL_WIDTH {
                    accu += &inner[row][col].get_row(cell_row);
                }
            }
        }
        accu
    }

    fn translate_board(&self, board: &Board) {
        let mut inner = self.board.borrow_mut();
        for row in 0..board.size() {
            for col in 0..board.size() {
                inner[row+1][col+1] = BoardCell::from(&board[Position::new(row, col)])
            }
        }
    }
}
