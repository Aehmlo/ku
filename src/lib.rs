//! A sudoku crate.

#![warn(
    missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results
)]

mod dimensions;
mod gen;
mod puzzle;
mod sol;
mod sudoku;

pub use gen::Generate;
pub use puzzle::Puzzle;
pub use sol::{Difficulty, Error as SolveError, Score, Solve};
pub use sudoku::{Element, Grid, Group, ParseError, Point, Sudoku};

pub use dimensions::DIMENSIONS;
