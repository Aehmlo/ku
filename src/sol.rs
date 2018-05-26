/// Represents the difficulty of a puzzle.
#[derive(Clone, Copy, Debug)]
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
pub enum Error {}

/// Trait defining a solvable puzzle.
pub trait Solve
where
    Self: Sized,
{
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
