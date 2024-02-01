use sudoku_solver::lib::sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new();
    sudoku.set(1, 2, 5);
    println!("{sudoku}");
}
