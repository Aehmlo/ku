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
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Difficulty {
    #[doc(hidden)]
    /// Filler
    Unplayable,
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

impl From<usize> for Difficulty {
    fn from(score: usize) -> Self {
        use Difficulty::*;
        match score {
            0...49 => Unplayable,
            50...150 => Beginner,
            151...250 => Easy,
            251...400 => Intermediate,
            401...550 => Difficult,
            _ => Advanced,
        }
    }
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
        self.score().map(|s| s.into())
    }
}

// TODO(#12): Allow higher orders (u128?)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PossibilitySet {
    pub values: u64,
}

impl PossibilitySet {
    /// Creates a new set full of possibilities.
    pub fn new(order: u8) -> Self {
        let mut values = 0;
        for i in 1..=order.pow(2) as usize {
            values |= 1 << (i - 1);
        }
        Self { values }
    }
    /// Elminates the given possible value from the set and returns the result.
    pub fn eliminate(&self, value: usize) -> Option<Self> {
        let values = self.values & !(1 << (value - 1));
        match values {
            0 => None,
            _ => Some(Self { values }),
        }
    }
    /// The number of possible values in this set.
    pub fn freedom(&self) -> usize {
        let mut x = self.values;
        let mut n = 0;
        while x > 0 {
            x &= x - 1;
            n += 1;
        }
        n
    }
    /// Whether the set contains the given possibility.
    pub fn contains(&self, value: usize) -> bool {
        self.values | (1 << (value - 1)) == self.values
    }
}

#[derive(Debug)]
pub struct PossibilityMap {
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
    // TODO(#10): Benchmark
    pub fn eliminate(&mut self, index: Point, value: usize) {
        self[index] = self[index].and_then(|e| e.eliminate(value));
    }

    // Returns the next easiest index to solve and its corresponding value.
    pub fn next(&self) -> (Option<Point>, Option<PossibilitySet>) {
        let mut best = None;
        let mut best_index = None;
        let mut best_score = None;
        for index in self.points() {
            if let Some(element) = self[index] {
                if best_score.is_none() || best_score.unwrap() > element.freedom() {
                    best = Some(element);
                    best_index = Some(index);
                    best_score = Some(element.freedom());
                }
            }
        }
        (best_index, best)
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
    match map.next() {
        (None, _) => {
            if context.problem.is_complete() {
                // We're done! Stash the solution and return.
                if context.count == 0 {
                    context.branch_score = difficulty;
                    context.solution = Some(context.problem.clone());
                }
                context.count += 1;
            }
            return;
        }
        (Some(index), Some(set)) => {
            let branch_factor = set.freedom() as isize - 1;
            let possible = (1..=(context.problem.order as usize).pow(2))
                .filter(|v| set.contains(*v))
                .collect::<Vec<_>>();
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
        _ => unreachable!(),
    }
}

/// Returns the number of empty cells in the passed sudoku.
///
/// Useful for scoring difficulty (see [Scoring](#Scoring)).
fn count_empty(sudoku: &Sudoku) -> usize {
    sudoku
        .elements
        .iter()
        .filter(|e| e.is_none())
        .collect::<Vec<_>>()
        .len()
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

    use sol::{calculate_c, Error, PossibilityMap, PossibilitySet, Solve};
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
        // TODO(#11): More cases
        let sudoku = Sudoku::new(3);
        let map: PossibilityMap = sudoku.into();
        for p in map.possibilities {
            assert_eq!(p, Some(PossibilitySet::new(3)));
        }
    }

    #[test]
    fn test_set_new() {
        let set = PossibilitySet::new(3);
        for i in 1..10 {
            assert!(set.contains(i));
        }
    }

    #[test]
    fn test_set_eliminate() {
        let mut set = PossibilitySet::new(3);
        for i in 1..9 {
            set = set.eliminate(i).unwrap();
            assert!(!set.contains(i));
        }
        assert_eq!(set.eliminate(9), None);
    }

    #[test]
    fn test_set_freedom() {
        let mut set = PossibilitySet::new(3);
        for i in 1..9 {
            set = set.eliminate(i).unwrap();
            assert_eq!(set.freedom(), 9 - i);
        }
    }
}
