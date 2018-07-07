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
#[cfg(feature = "use_rand")]
extern crate rand;
#[cfg_attr(feature = "use_stdweb", macro_use)]
#[cfg(feature = "use_stdweb")]
extern crate stdweb;

mod dimensions;
mod gen;
mod puzzle;
mod sol;
mod sudoku;

#[cfg(feature = "ui")]
extern crate num_traits;
#[cfg(feature = "ui")]
pub mod ui;

pub use gen::Generate;
pub use puzzle::Puzzle;
pub use sol::{Difficulty, Error as SolveError, Score, Solve};
pub use sudoku::{Element, Grid, Group, ParseError, Point, Sudoku};

pub use dimensions::DIMENSIONS;
