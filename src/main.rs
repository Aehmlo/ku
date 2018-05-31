#[macro_use]
extern crate clap;
extern crate sudoku;

use std::{
    fs::File, io::{stdin, Error as IoError, Read},
};

use sudoku::{ParseError, Solve, SolveError, Sudoku};

#[derive(Debug)]
enum Error {
    SolveError(SolveError),
    ParseError(ParseError),
    IoError(IoError),
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::ParseError(error)
    }
}

impl From<SolveError> for Error {
    fn from(error: SolveError) -> Self {
        Error::SolveError(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::IoError(error)
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() -> Result<(), Error> {
    let matches = clap_app!(ku =>
        (setting: clap::AppSettings::ArgRequiredElseHelp)
        (setting: clap::AppSettings::VersionlessSubcommands)
        (about: "A sudoku generator/solver/manipulator.")
        (@subcommand solve =>
            (about: "Solves the given puzzle.")
            (@arg INPUT: "Sets the input file (defaults to stdin).")
        )
    ).get_matches();
    if let Some(matches) = matches.subcommand_matches("solve") {
        let solution = attempt_solve(&matches)?;
        println!("{}", solution);
    }
    Ok(())
}

fn attempt_solve(matches: &clap::ArgMatches) -> Result<Sudoku, Error> {
    let mut reader: Box<Read> = if matches.is_present("INPUT") {
        Box::new(File::open(matches.value_of("INPUT").unwrap()).expect("File not found."))
    } else {
        Box::new(stdin())
    };
    let mut puzzle = String::new();
    reader.read_to_string(&mut puzzle)?;
    let puzzle: Sudoku = puzzle.parse()?;
    puzzle.solution().map_err(|e| e.into())
}
