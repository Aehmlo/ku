extern crate sudoku;
use sudoku::{Grid, Solve, Sudoku};

#[cfg_attr(feature = "2D", test)]
#[cfg(feature = "2D")]
fn test_solve_o3_2d() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O3.txt")
        .parse()
        .unwrap();
    let solution = puzzle.solution();
    assert!(solution.is_ok());
}

#[cfg_attr(feature = "2D", test)]
#[cfg_attr(feature = "2D", ignore)]
#[cfg(feature = "2D")]
fn test_solve_o4_2d() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O4.txt")
        .parse()
        .unwrap();
    let solution = puzzle.solution();
    assert!(solution.is_ok());
}

#[cfg_attr(feature = "2D", test)]
#[cfg_attr(feature = "2D", ignore)]
#[cfg(feature = "2D")]
fn test_uniquely_solveable() {
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O3.txt")
        .parse()
        .unwrap();
    assert!(puzzle.is_uniquely_solvable());
    let puzzle: Sudoku = include_str!("../tests/sudokus/solvable/2D-O4.txt")
        .parse()
        .unwrap();
    assert!(puzzle.is_uniquely_solvable());
}

#[cfg_attr(feature = "2D", test)]
#[cfg(feature = "2D")]
fn test_group_is_complete_and_is_valid() {
    let solution = include_str!("../tests/sudokus/solvable/2D-O3.txt")
        .parse::<Sudoku>()
        .unwrap()
        .solution()
        .unwrap();
    for point in solution.points() {
        for group in solution.groups(point).iter() {
            assert!(group.is_complete());
            assert!(group.is_valid());
        }
    }
}
