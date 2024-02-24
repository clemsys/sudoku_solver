#[derive(Copy, Clone)]
pub enum Cell {
    Value(u8),
    Allowed([bool; 9]),
}

type Board = [[Cell; 9]; 9];

#[derive(Clone)]
pub struct Sudoku(Board);

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Deref for Sudoku {
    type Target = Board;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Sudoku {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Index<(usize, usize)> for Sudoku {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Sudoku {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl Sudoku {
    pub const fn new() -> Self {
        Self([[Cell::Allowed([true; 9]); 9]; 9])
    }

    pub fn set(&mut self, index: (usize, usize), value: u8) -> Result<(), ()> {
        if let Cell::Allowed(p) = self[index] {
            if p[(value - 1) as usize] {
                self[index] = Cell::Value(value);
                self.update_peers_on_set(index, value);
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn get(&self, index: (usize, usize)) -> Cell {
        self[index]
    }

    fn update_peers_on_set(&mut self, index: (usize, usize), value: u8) {
        for peer in Self::peers(index) {
            if let Cell::Allowed(mut p) = self[peer] {
                p[(value - 1) as usize] = false;
                self[peer] = Cell::Allowed(p);
            }
        }
    }

    /// Returns the cells with the cell which is the most constrained by its peers
    /// as well as the number of allowed values for that cell.
    pub(super) fn least_possibilities(&self) -> Option<((usize, usize), usize)> {
        let mut min_count = 10;
        let mut index = None;
        'outer: for i in 0..9 {
            for j in 0..9 {
                if let Cell::Allowed(p) = self[(i, j)] {
                    let count = p.iter().filter(|b| **b).count();
                    if count < min_count {
                        min_count = count;
                        index = Some((i, j));
                    }
                    if count == 0 {
                        break 'outer;
                    }
                }
            }
        }
        match index {
            Some(i) => Some((i, min_count)),
            None => None,
        }
    }

    fn peers(index: (usize, usize)) -> [(usize, usize); 20] {
        let mut peers = [index; 20];
        for i in 0..8 {
            peers[i] = (index.0, i + usize::from(i >= index.1));
            peers[i + 8] = (i + usize::from(i >= index.0), index.1);
        }
        let top_left = ((index.0 / 3) * 3, (index.1 / 3) * 3);
        for i in 0..2 {
            for j in 0..2 {
                peers[i * 2 + j + 16] = (
                    top_left.0 + ((index.0 % 3) + i + 1) % 3,
                    top_left.1 + ((index.1 % 3) + j + 1) % 3,
                );
            }
        }
        peers
    }
}

impl std::str::FromStr for Sudoku {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sudoku = Self::new();
        let valid_chars: Vec<char> = s
            .chars()
            .filter(|c| c.is_ascii_digit() || c == &'.')
            .collect();
        for (i, c) in valid_chars.iter().enumerate() {
            if let Some(v) = c.to_digit(10) {
                let index = (i / 9, i % 9);
                sudoku.set(index, u8::try_from(v).unwrap())?; // digits are in 0-9 so they fit in u8
            }
        }
        Ok(sudoku)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Value(v) => write!(f, "{v}"),
            Cell::Allowed(_) => write!(f, "·"),
        }
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const LINE_SEP: &str = "+-------+-------+-------+";

        let mut s = String::new();

        for (i, row) in self.iter().enumerate() {
            if i % 3 == 0 {
                s.push_str(LINE_SEP);
                s.push('\n');
            }

            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    s.push_str("| ");
                }

                s.push_str(&format!("{cell} "));
            }

            s.push_str("|\n");
        }

        s.push_str(LINE_SEP);

        write!(f, "{s}")
    }
}
