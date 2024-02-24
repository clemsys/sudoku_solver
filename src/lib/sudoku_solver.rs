use super::sudoku::Sudoku;

pub fn solve(sudoku: &Sudoku) -> Option<Sudoku> {
    match sudoku.least_possibilities() {
        None => Some(sudoku.clone()), // already solved
        Some((index, possibilities)) => {
            if possibilities == 0 {
                return None;
            }
            for value in 1..10 {
                let mut s = sudoku.clone();
                if s.set(index, value).is_ok() {
                    if let Some(solved) = solve(&s) {
                        return Some(solved);
                    }
                }
            }
            None
        }
    }
}
