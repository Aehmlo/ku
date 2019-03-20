#[macro_use]
extern crate clap;
extern crate sudoku;

use std::{
    fs::File,
    io::{stdin, Error as IoError, Read},
};

use sudoku::{Difficulty, Generate, ParseError, Score, Solve, SolveError, Sudoku};

#[derive(Debug)]
enum Error {
    Solve(SolveError),
    Parse(ParseError),
    Io(IoError),
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Error::Parse(error)
    }
}

impl From<SolveError> for Error {
    fn from(error: SolveError) -> Self {
        Error::Solve(error)
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::Io(error)
    }
}

fn puzzle(matches: &clap::ArgMatches) -> Result<Sudoku, Error> {
    let mut reader: Box<Read> = if matches.is_present("INPUT") {
        Box::new(File::open(matches.value_of("INPUT").unwrap()).expect("File not found."))
    } else {
        Box::new(stdin())
    };
    let mut puzzle = String::new();
    reader.read_to_string(&mut puzzle)?;
    puzzle.parse().map_err(Into::into)
}

#[rustfmt::skip]
fn main() -> Result<(), Error> {
    let matches = clap_app!(ku =>
        (setting: clap::AppSettings::ArgRequiredElseHelp)
        (setting: clap::AppSettings::VersionlessSubcommands)
        (about: "A sudoku generator/solver/manipulator.")
        (@subcommand solve =>
            (about: "Solves the given sudoku.")
            (@arg INPUT: "Sets the input file (defaults to stdin).")
        )
        (@subcommand score =>
            (about: "Scores the given sudoku.")
            (@arg INPUT: "Sets the input file (defaults to stdin).")
        )
        (@subcommand generate =>
            (about: "Generates a sudoku.")
            (@arg ORDER: "The order of sudoku to be generated (defaults to 3).")
        )
    ).get_matches();
    if let Some(matches) = matches.subcommand_matches("solve") {
        let solution = solve(&matches)?;
        println!("{}", solution);
    } else if let Some(matches) = matches.subcommand_matches("score") {
        if let Some(score) = score(&matches) {
            println!("Score: {}", score);
        } else {
            println!("Couldn't score puzzle.");
        }
    } else if let Some(matches) = matches.subcommand_matches("generate") {
        let order = matches.value_of("ORDER").and_then(|s: &str| s.parse().ok()).unwrap_or(3);
        println!("{:X}", Sudoku::generate(order, Difficulty::Beginner));
    }
    Ok(())
}

fn solve(matches: &clap::ArgMatches) -> Result<Sudoku, Error> {
    puzzle(matches).and_then(|p| p.solution().map_err(Into::into))
}

fn score(matches: &clap::ArgMatches) -> Option<usize> {
    puzzle(matches).ok().and_then(|p| p.score())
}
