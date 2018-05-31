extern crate sudoku;
use sudoku::{Solve, Sudoku};

#[cfg_attr(feature = "2D", test)]
#[cfg(feature = "2D")]
fn test_solve_O3_2D() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O3.txt")
        .parse()
        .unwrap();
    let solution = puzzle.solution();
    assert!(solution.is_ok());
}

#[cfg_attr(feature = "2D", test)]
#[cfg(feature = "2D")]
fn test_solve_O4_2D() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O4.txt")
        .parse()
        .unwrap();
    let solution = puzzle.solution();
    assert!(solution.is_ok());
}

#[cfg_attr(feature = "2D", test)]
#[cfg(feature = "2D")]
fn test_uniquely_solveable() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O4.txt")
        .parse()
        .unwrap();
    assert!(puzzle.is_uniquely_solvable());
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O4.txt")
        .parse()
        .unwrap();
    assert!(puzzle.is_uniquely_solvable());
}
