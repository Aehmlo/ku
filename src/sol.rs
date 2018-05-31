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
use sudoku::Grid;
use Element;
use Point;
use Sudoku;
use DIMENSIONS;

use std::ops::{Index, IndexMut};

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
    /// A mere placeholder; this will be replaced by proper errors in a future revision.
    Unknown,
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
    fn score(&self) -> Option<usize>;
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

#[derive(Clone, Debug, PartialEq)]
struct PossibilitySet {
    values: Vec<usize>,
}

impl PossibilitySet {
    /// Creates a new set full of possibilities.
    pub fn new(order: u8) -> Self {
        let values = (1..((order.pow(2) as usize) + 1)).collect();
        Self { values }
    }
    /// Elminates the given possible value from the set and returns the result.
    pub fn eliminate(&self, value: usize) -> Option<Self> {
        // Don't bother checking for value; we'd clone almost all the values anyway.
        let values = self
            .values
            .clone()
            .into_iter()
            .filter(|v| v != &value)
            .collect::<Vec<_>>();
        match values.len() {
            0 => None,
            _ => Some(Self { values }),
        }
    }
    /// The number of possible values in this set.
    pub fn freedom(&self) -> usize {
        self.values.len()
    }
}

#[derive(Debug)]
struct PossibilityMap {
    possibilities: Vec<Option<PossibilitySet>>,
    order: u8,
}

impl PossibilityMap {
    /// Constructs a blank possibilitiy map of the given order.
    pub fn new(order: u8) -> Self {
        Self {
            possibilities: vec![
                Some(PossibilitySet::new(order));
                (order as usize).pow(2 + DIMENSIONS as u32)
            ],
            order,
        }
    }

    /// Removes the given value from the set of possibilities at the given location.
    // There's no way it's cheaper to reconstruct the map each time, so we make this mutating.
    // TODO: Benchmark
    pub fn eliminate(&mut self, index: Point, value: usize) {
        let current = self[index].clone();
        match current {
            None => {}
            Some(set) => {
                self[index] = set.eliminate(value);
            }
        }
    }

    // Returns the next easiest index to solve.
    pub fn next_index(&self) -> Option<Point> {
        let mut best = None;
        let mut best_index = None;
        for index in self.points() {
            if let Some(ref element) = self[index] {
                if best.is_none() {
                    best = Some(element.freedom());
                    best_index = Some(index);
                } else if best.unwrap() > element.freedom() {
                    best = Some(element.freedom());
                    best_index = Some(index);
                }
            }
        }
        best_index
    }
}

impl Index<Point> for PossibilityMap {
    type Output = Option<PossibilitySet>;

    fn index(&self, index: Point) -> &Self::Output {
        let index = index.fold(self.order);
        &self.possibilities[index]
    }
}

impl IndexMut<Point> for PossibilityMap {
    fn index_mut(&mut self, index: Point) -> &mut Option<PossibilitySet> {
        let index = index.fold(self.order);
        &mut self.possibilities[index]
    }
}

impl Grid for PossibilityMap {
    fn points(&self) -> Vec<Point> {
        (0..(self.order as usize).pow(2 + DIMENSIONS as u32))
            .map(|p| Point::unfold(p, self.order))
            .collect()
    }
}

impl From<Sudoku> for PossibilityMap {
    fn from(sudoku: Sudoku) -> Self {
        let order = sudoku.order;
        let mut map = PossibilityMap::new(order);
        for i in 0..(sudoku.order as usize).pow(2 + DIMENSIONS as u32) {
            let point = Point::unfold(i, order);
            if sudoku[point].is_some() {
                map[point] = None;
            } else {
                let groups = sudoku.groups(point);
                for group in groups.iter() {
                    let elements = group.elements();
                    for element in elements {
                        match element {
                            Some(Element(value)) => {
                                map.eliminate(point, value as usize);
                            }
                            None => {}
                        }
                    }
                }
            }
        }
        map
    }
}

pub fn solve(puzzle: &Sudoku) -> Result<Sudoku, Error> {
    solve_and_score(puzzle).map(|(sol, _)| sol)
}

pub fn solve_and_score(puzzle: &Sudoku) -> Result<(Sudoku, usize), Error> {
    let mut context = Context {
        problem: puzzle.clone(),
        count: 0,
        solution: None,
        branch_score: 0,
    };
    recurse(&mut context, 0);
    let s = context.branch_score;
    let c = calculate_c(puzzle) as isize;
    let e = count_empty(puzzle) as isize;
    context
        .solution
        .ok_or(Error::Unknown)
        .map(|sol| (sol, (s * c + e) as usize))
}

struct Context {
    problem: Sudoku,
    count: usize,
    solution: Option<Sudoku>,
    branch_score: isize,
}

fn recurse(mut context: &mut Context, difficulty: isize) {
    let problem = context.problem.clone();
    let map: PossibilityMap = problem.into();
    match map.next_index() {
        None => {
            // We're done! Stash the solution and return.
            if context.count == 0 {
                context.branch_score = difficulty;
                context.solution = Some(context.problem.clone());
            }
            context.count += 1;
            return;
        }
        Some(index) => {
            let set = map[index].clone().unwrap();
            let branch_factor = set.freedom() as isize - 1;
            let possible = set.values;
            let difficulty = difficulty + branch_factor.pow(DIMENSIONS as u32);
            for value in possible {
                let problem = context
                    .problem
                    .substitute(index, Some(Element(value as u8)));
                context.problem = problem;
                recurse(&mut context, difficulty);
                if context.count > 1 {
                    // There are multiple solutions; abort.
                    return;
                }
            }
            context.problem = context.problem.substitute(index, None);
        }
    }
}

/// Returns the number of empty cells in the passed sudoku.
///
/// Useful for scoring difficulty (see [Scoring](#Scoring)).
fn count_empty(sudoku: &Sudoku) -> usize {
    sudoku
        .elements
        .clone()
        .iter()
        .filter(|e| e.is_none())
        .count()
}

/// Calculates the value of `C`, as discussed in [Scoring](#Scoring).
fn calculate_c(sudoku: &Sudoku) -> usize {
    let order = sudoku.order;
    10.0_f64.powf((order as f64).powf(4.0).log10().ceil()) as usize
}

/// Scores the passed, if it's solvable.
pub fn score(sudoku: &Sudoku) -> Option<usize> {
    solve_and_score(&sudoku).ok().map(|(_, s)| s)
}

#[cfg(test)]
mod tests {

    use sol::{calculate_c, Difficulty, Error, PossibilityMap, PossibilitySet, Score, Solve};
    use Point;
    use Sudoku;
    use DIMENSIONS;

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
        difficulty: usize,
    }

    impl Solve for DifficultyDummyPuzzle {
        fn solution(&self) -> Result<Self, Error> {
            Err(Error::__TestOther)
        }
    }

    impl Score for DifficultyDummyPuzzle {
        fn score(&self) -> Option<usize> {
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
        assert_eq!(calculate_c(&sudoku), 100);
        let sudoku = Sudoku::new(4);
        assert_eq!(calculate_c(&sudoku), 1_000);
        let sudoku = Sudoku::new(5);
        assert_eq!(calculate_c(&sudoku), 1_000);
        let sudoku = Sudoku::new(6);
        assert_eq!(calculate_c(&sudoku), 10_000);
    }

    #[test]
    fn test_map_new() {
        for order in 1..6 {
            let map = PossibilityMap::new(order);
            for i in 0..(order as usize).pow(DIMENSIONS as u32 + 2) {
                let index = Point::unfold(i, order);
                let set = PossibilitySet::new(order);
                assert_eq!(map[index], Some(set));
            }
        }
    }

    #[test]
    fn test_map_from_sudoku() {
        // TODO: More cases
        let sudoku = Sudoku::new(3);
        let map: PossibilityMap = sudoku.into();
        for p in map.possibilities {
            assert_eq!(p, Some(PossibilitySet::new(3)));
        }
    }
}
