//! A sudoku crate.

#![warn(
    missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unused_extern_crates, unused_import_braces, unused_qualifications,
    unused_results
)]

mod gen;
mod puzzle;
mod sol;
mod sudoku;

pub use gen::Generate;
pub use puzzle::Puzzle;
pub use sol::{Difficulty, Score, Solve};
pub use sudoku::{Element, Group, Point, Sudoku};

/// The number of dimensions in which all sudoku methods will operate.
pub const DIMENSIONS: usize = 2; // We may allow changing this later.
