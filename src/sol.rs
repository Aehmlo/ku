//! Items for solving a sudoku.
//!
//! The general algorithm is based on
//! [that proposed by Daniel Beer](https://dlbeer.co.nz/articles/sudoku.html).
//!
//! # Approach
//!
//! We first find the empty element with the fewest possible values. We then try each of these
//! candidates, recursively solving from there until we (hopefully) find a solution.
//!
//! We search until we've either found two solutions (meaning that the puzzle is not uniquely
//! solvable) or exhausted the search tree.
//!
//! # Scoring
//! During the solving process, we calculate a *branch-difficulty score*, `S = Σ((B - 1)²)`, where
//! `B` is the branching factor at a given node in the search tree from root to solution.
//!
//! If no backtracking occurs, the branch-difficulty score is 0.
//!
//! ## Tabulation
//! The final difficulty score is given by `D = S * C + E`, where `C` is the first power of 10
//! greater than the number of elements and `E` is the number of empty elements.
use Sudoku;

/// Represents the difficulty of a puzzle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Difficulty {
    /// Very easy puzzles, ideal for learning a new game.
    Beginner,
    /// More advanced puzzles, good for practicing a new game.
    Easy,
    /// Intermediate puzzles, good for a casual puzzle-solving session.
    Intermediate,
    /// Advanced, thought-provoking puzzles.
    Difficult,
    /// Coffee shop puzzles.
    Advanced,
}

/// Encodes errors encountered while attempting a puzzle solution.
#[derive(Clone, Debug)]
#[allow(missing_copy_implementations)] // This is an error type.
pub enum Error {
    #[doc(hidden)]
    __TestOther,
}

/// Trait defining a solvable puzzle.
pub trait Solve: Sized {
    /// Returns the puzzle's unique solution if it exists.
    fn solution(&self) -> Result<Self, Error>;
    /// Whether the puzzle has a unique solution.
    fn is_uniquely_solvable(&self) -> bool {
        self.solution().is_ok()
    }
}

/// Trait defining a puzzle with quantifiable difficulty.
pub trait Score: Solve {
    /// The raw difficulty score of this puzzle.
    fn score(&self) -> Option<u16>;
    /// The graded difficulty score of this puzzle.
    fn difficulty(&self) -> Option<Difficulty> {
        use self::Difficulty::*;
        self.score().map(|score| match score {
            0...50 => Beginner,
            51...125 => Easy,
            126...200 => Intermediate,
            201...300 => Difficult,
            _ => Advanced,
        })
    }
}

struct Context {
    problem: Sudoku,
    count: usize,
    solution: Option<Sudoku>,
    branch_score: usize,
}

mod calc {
    use Puzzle;
    use Sudoku;

    /// Calculates the value of `C`, as discussed in [Scoring](#Scoring).
    pub fn c(sudoku: &Sudoku) -> usize {
        let order = sudoku.order();
        10.0_f64.powf((order as f64).powf(4.0).log10().ceil()) as usize
    }

    /// Calculates the value of `D`, as discussed in [Scoring](#Scoring).
    pub fn difficulty(s: usize, c: usize, e: usize) -> usize {
        s * c + e
    }
}

#[cfg(test)]
mod tests {

    use sol::{
        calc::{c, difficulty}, Difficulty, Error, Score, Solve,
    };
    use Sudoku;

    struct DummyPuzzle(bool);

    impl DummyPuzzle {
        fn new(solvable: bool) -> Self {
            Self { 0: solvable }
        }
    }

    impl Solve for DummyPuzzle {
        fn solution(&self) -> Result<Self, Error> {
            if self.0 {
                Ok(Self { 0: true })
            } else {
                Err(Error::__TestOther)
            }
        }
    }

    #[test]
    fn test_is_uniquely_solvable() {
        let solvable = DummyPuzzle::new(true);
        assert_eq!(solvable.is_uniquely_solvable(), true);
        let unsolvable = DummyPuzzle::new(false);
        assert_eq!(unsolvable.is_uniquely_solvable(), false);
    }

    struct DifficultyDummyPuzzle {
        difficulty: u16,
    }

    impl Solve for DifficultyDummyPuzzle {
        fn solution(&self) -> Result<Self, Error> {
            Err(Error::__TestOther)
        }
    }

    impl Score for DifficultyDummyPuzzle {
        fn score(&self) -> Option<u16> {
            Some(self.difficulty)
        }
    }

    #[test]
    fn test_score_difficulty() {
        let scores = [0, 50, 51, 125, 126, 200, 201, 300, 301, 999];
        for difficulty in scores.into_iter() {
            let difficulty = *difficulty;
            let puzzle = DifficultyDummyPuzzle { difficulty };
            if difficulty < 51 {
                assert_eq!(puzzle.difficulty().unwrap(), Difficulty::Beginner)
            } else if difficulty < 126 {
                assert_eq!(puzzle.difficulty().unwrap(), Difficulty::Easy)
            } else if difficulty < 201 {
                assert_eq!(puzzle.difficulty().unwrap(), Difficulty::Intermediate)
            } else if difficulty < 301 {
                assert_eq!(puzzle.difficulty().unwrap(), Difficulty::Difficult)
            } else {
                assert_eq!(puzzle.difficulty().unwrap(), Difficulty::Advanced)
            }
        }
    }

    #[test]
    fn test_calculate_c() {
        let sudoku = Sudoku::new(3);
        assert_eq!(c(&sudoku), 100);
        let sudoku = Sudoku::new(4);
        assert_eq!(c(&sudoku), 1_000);
        let sudoku = Sudoku::new(5);
        assert_eq!(c(&sudoku), 1_000);
        let sudoku = Sudoku::new(6);
        assert_eq!(c(&sudoku), 10_000);
    }

    #[test]
    fn test_calculate_difficulty() {
        for s in 0..70 {
            for c in 2..5 {
                for e in 0..50 {
                    assert_eq!(difficulty(s, 10usize.pow(c), e), s * 10usize.pow(c) + e);
                }
            }
        }
    }
}
