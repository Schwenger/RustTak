use crate::actions::Action;
use crate::board::piece::{Stack, Piece, PieceKind};
use crate::board::{Position, Board, Direction};
use crate::player::Color;
use colored::*;
use std::cell::{RefCell, Ref};
use std::ops::Range;

pub(crate) trait CLHumanDisplay {
    fn cl_display(&self) -> String;
}

impl CLHumanDisplay for Action {
    fn cl_display(&self) -> String {
        match self {
            Action::Place(pos, kind) => format!("placed a {} at {}", pos.cl_display(), kind.cl_display()),
            Action::Slide(src, dir, v) if v.len() == 1 =>
                format!("moved {} pieces from {} {}", v[0], src.cl_display(), dir.cl_display()),
            Action::Slide(src, dir, v) => {
                let takes = v.iter().map(|n| format!("{}", n)).collect::<Vec<String>>().join(", ");
                format!("started sliding {} from {} taking {} pieces", dir.cl_display(), src.cl_display(), takes)
            }
        }
    }
}

impl CLHumanDisplay for PieceKind {
    fn cl_display(&self) -> String {
        match self {
            PieceKind::Stone => String::from("🝙"),
            PieceKind::StandingStone => String::from("╳"),
            PieceKind::CapStone => String::from(/*"♜"*/ "👑"),
        }
    }
}

impl CLHumanDisplay for Position {
    fn cl_display(&self) -> String {
        let c: u8 = 65u8 + (self.col as u8);
        format!("({}, {})", self.row, c as char)
    }
}

impl CLHumanDisplay for Direction {
    fn cl_display(&self) -> String {
        match self {
            Direction::North => String::from("upwards"),
            Direction::South => String::from("downwards"),
            Direction::East  => String::from("to the right"),
            Direction::West  => String::from("to the left"),
        }
    }
}

impl CLHumanDisplay for Piece {
    fn cl_display(&self) -> String {
        match self.color {
            Color::Red => self.kind.cl_display().red().to_string(),
            Color::Blk => self.kind.cl_display().black().to_string(),
        }

    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoardCell {
    inner: Vec<Vec<String>>,
}

// TODO: Should be odd.
// TODO: Scale by max stack size!
// TODO: Maybe add inner borders.
const CELL_WIDTH: usize = 3;
impl BoardCell {

    fn default() -> BoardCell {
        BoardCell { inner: vec![Vec::new()] }
    }

    fn empty() -> BoardCell {
        BoardCell { inner: vec![Self::empty_row(); CELL_WIDTH] }
    }

    fn straight() -> BoardCell {
        BoardCell { inner: vec![vec!["-".bold().to_string(); CELL_WIDTH]] }
    }

    fn pipe() -> BoardCell {
        BoardCell { inner: vec![vec!["|".bold().to_string()]; CELL_WIDTH] }
    }

    fn cross() -> BoardCell {
        BoardCell { inner: vec![vec!["┼".bold().to_string()]] }
    }

    fn from(s: &Stack) -> BoardCell {
        let mut s = s.clone(); // TODO: Avoid this clone.
        let num_printable = CELL_WIDTH * CELL_WIDTH;
        let truncate = s.len() > num_printable;
        if truncate {
            s.peek_from_top(num_printable - 1);
            unimplemented!()
        }
        let num_rows = (s.len() as f32 / CELL_WIDTH as f32).ceil() as usize;

        let sieve = |_: usize| -> Stack {
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

    fn get_row(&self, i: usize) -> String {
        if i < self.inner.len() {
            self.inner[i].iter().fold(String::new(), |s1, s2| s1 + &s2)
        } else {
            String::new() // We're already fully printed.
        }
    }

    fn empty_row() -> Vec<String> {
        vec![String::from(" "); CELL_WIDTH]
    }

    fn distribute_rows(rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
        assert!(rows.len() < CELL_WIDTH);
        let space_left_over = CELL_WIDTH - rows.len();
        let space_top = space_left_over / 2;
        (0..CELL_WIDTH).map(|ix| {
            if ix > space_top && ix < space_top + rows.len() {
                rows[ix - space_top].clone() // TODO: This clone is avoidable.
            } else {
                Self::empty_row()
            }
        }).collect()
    }

    fn transform_single_row_stack(s: Stack) -> Vec<String> {
        assert!(s.len() <= CELL_WIDTH);
        let mut row = vec![String::from(" "); CELL_WIDTH];
        let leftover_space = CELL_WIDTH - s.len();
        let space_left = leftover_space / 2;
        for (i, piece) in s.iter().enumerate() {
            row[i + space_left] = piece.cl_display();
        }
        row
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CIBoardPrinter {
    board: RefCell<Vec<Vec<BoardCell>>>,
    size: usize,
}

impl CIBoardPrinter {

    fn header_row(&self) -> usize { 0 }
    fn footer_row(&self) -> usize { self.size - 1 }
    fn left_border(&self) -> usize { 0 }
    fn right_boarder(&self) -> usize { self.size - 1 }

    fn inner_range(&self) -> Range<usize> {
        (1..(self.size - 1)) // exclude border/header/footer
    }

    fn full_range(&self) -> Range<usize> {
        (0..self.size)
    }

    fn inner_to_pos(&self, row: usize, col: usize) -> Position {
        Position::new(row - 1, col - 1)
    }

    pub(crate) fn new(board_size: usize) -> CIBoardPrinter {
        let size = board_size + 2;
        let board = vec![vec![BoardCell::default(); size]; size];
        let printer = CIBoardPrinter { board: RefCell::new(board), size };
        let mut board = printer.board.borrow_mut();
        board[printer.header_row()][printer.left_border()] = BoardCell::cross();
        board[printer.footer_row()][printer.left_border()] = BoardCell::cross();
        board[printer.header_row()][printer.right_boarder()] = BoardCell::cross();
        board[printer.footer_row()][printer.right_boarder()] = BoardCell::cross();
        for ix in printer.inner_range() {
            board[printer.header_row()][ix] = BoardCell::straight();
            board[printer.footer_row()][ix] = BoardCell::straight();
            board[ix][printer.left_border()] = BoardCell::pipe();
            board[ix][printer.right_boarder()] = BoardCell::pipe();
            for j in printer.inner_range() {
                board[ix][j] = BoardCell::empty();
            }
        }
        drop(board); // Drop borrow so we can move printer.
        printer
    }

    fn print_header_footer(&self, cells: &Ref<Vec<Vec<BoardCell>>>, header: bool) -> String {
        let row = if header { self.header_row() } else { self.footer_row() };
        let mut accu = String::new();
        for col in self.full_range() {
            accu += &cells[row][col].get_row(0);
        }
        accu
    }

    pub(crate) fn print(&self, board: &Board) -> String {
        self.translate_board(board);
        let mut accu = String::new();
        let cells = self.board.borrow();
        accu += &self.print_header_footer(&cells, true);
        accu += "\n";
        for row in self.inner_range() {
            for cell_row in 0..CELL_WIDTH {
                for col in self.full_range() {
                    accu += &cells[row][col].get_row(cell_row);
                }
                accu += "\n";
            }
        }
        accu += &self.print_header_footer(&cells, false);
        accu
    }

    fn translate_board(&self, board: &Board) {
        let mut cells = self.board.borrow_mut();
        for row in self.inner_range() {
            for col in self.inner_range() {
                cells[row][col] = BoardCell::from(&board[self.inner_to_pos(row, col)])
            }
        }
    }

}

#[cfg(test)]
mod tests {

    use crate::board::Board;
    use super::CIBoardPrinter;
    use colored::*;
    use crate::board::piece::*;
    use crate::board::Position;
    use crate::player::Color;

    fn setup(size: usize) -> (Board, CIBoardPrinter) {
        (Board::new(size), CIBoardPrinter::new(size))
    }

    fn bold(s: &str) -> String {
        s.chars().map(|c| c.to_string().bold().to_string()).collect()
    }

    #[test]
    fn test_empty() {
        let (board, printer) = setup(2);
        let was = printer.print(&board);
        let expected = vec![
        bold("┼------┼"),
        bold("|") + &"      " + &bold("|"),
        bold("|") + &"      " + &bold("|"),
        bold("|") + &"      " + &bold("|"),
        bold("|") + &"      " + &bold("|"),
        bold("|") + &"      " + &bold("|"),
        bold("|") + &"      " + &bold("|"),
        bold("┼------┼"),
        ].join("\n");
        assert_eq!(was, expected);
    }

    #[test]
    fn test_single() {
        let (mut board, printer) = setup(2);
        board[Position::new(1,1)] += Stack::from(vec![Piece::new(PieceKind::Stone, Color::Red)]);
        println!("{}", printer.print(&board));
        let was = printer.print(&board);
        let expected = vec![
            bold("┼------┼"),
            bold("|") + &"      " + &bold("|"),
            bold("|") + &"      " + &bold("|"),
            bold("|") + &"      " + &bold("|"),
            bold("|") + &"      " + &bold("|"),
            bold("|") + &"      " + &bold("|"),
            bold("|") + &"      " + &bold("|"),
            bold("┼------┼"),
        ].join("\n");
        assert_eq!(was, expected);
    }
}