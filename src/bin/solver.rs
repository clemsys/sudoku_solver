use std::io::{self, Read};
use std::process;
use std::str::FromStr;
use sudoku_solver::lib::sudoku::Sudoku;
use sudoku_solver::lib::sudoku_solver::solve;

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap_or_else(|_| {
        eprintln!("Error reading stdin");
        process::exit(1);
    });
    let sudoku = Sudoku::from_str(&s).unwrap_or_else(|_| {
        eprintln!("Error parsing sudoku");
        process::exit(2);
    });
    solve(&sudoku).map_or_else(|| println!("Solution not found"), |s| println!("{s}"));
}
