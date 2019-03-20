//! A sudoku crate.

#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![deny(missing_docs)]

#[cfg(all(feature = "use_stdweb", feature = "use_rand"))]
compile_error!("use_stdweb and use_rand are mutually exclusive.");

mod dimensions;
mod gen;
mod puzzle;
mod sol;
mod sudoku;

#[cfg(feature = "ui")]
extern crate num_traits;
#[cfg(feature = "ui")]
pub mod ui;

pub use crate::gen::Generate;
pub use crate::puzzle::Puzzle;
pub use crate::sol::{Difficulty, Error as SolveError, Score, Solve};
pub use crate::sudoku::{Element, Grid, Group, ParseError, Point, Sudoku};

pub use crate::dimensions::DIMENSIONS;
