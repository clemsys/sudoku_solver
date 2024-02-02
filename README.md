# Sudoku Solver

Author: [Clément CHAPOT](mailto:clement@chapot.ovh) <br>
Description: simple sudoku solver based on backtracking, packed in a library with its own CLI

## Building

Build the project using `cargo build --release`.

## Running

Run using `./target/release/solver` from the root of the repository.

`solver` reads a sudoku from `stdin`. All characters from the input string are ignored, except `[1-9]` and `.`, which are used to indicate cells whose value has to be found by the solver.

## Project structure

The core of the project lies in `src/lib/`.

`src/lib/sudoku.rs` defines a sudoku `struct`, which is guaranteed to always represent a valid sudoku state, which then simplifies the work of the solver defined in `src/lib/sudoku.rs`.

## Performance

This solver might not be the most optimized, but it is able to solve any sudoku in about 1ms.
