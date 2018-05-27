use Difficulty;
use Score;

/// Trait to generate a puzzle.
///
/// Requires that the puzzle be solvable (to ensure the desired difficulty is
/// attained).
pub trait Generate: Score + Sized {
    /// Generates a puzzle of the desired order and difficulty.
    fn generate(order: u8, difficulty: Difficulty) -> Self {
        unimplemented!() // TODO: Default puzzle generation based on Solve
    }
}
