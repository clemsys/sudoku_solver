use std::fmt::Display;

#[derive(Copy, Clone)]
enum Cell {
    Value(u8),
    Possible([bool; 9]),
}

pub struct Sudoku {
    board: [[Cell; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            board: [[Cell::Possible([true; 9]); 9]; 9],
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        if let Cell::Possible(p) = self.board[row][col] {
            if p[value as usize] {
                self.board[row][col] = Cell::Value(value);
            } else {
                panic!("Value {value} is not authorized in cell ({row}, {col})");
            }
        } else {
            panic!("Cell already set");
        }
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LINE_SEP: &str = "+-------+-------+-------+\n";

        let mut s = String::new();

        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 {
                s.push_str(LINE_SEP);
            }

            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    s.push_str("| ");
                }

                match cell {
                    Cell::Value(v) => s.push_str(&format!("{v} ")),
                    Cell::Possible(_) => s.push_str("· "),
                }
            }

            s.push_str("|\n");
        }

        s.push_str(LINE_SEP);

        write!(f, "{}", s)
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}
