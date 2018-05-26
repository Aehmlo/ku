/// Includes information about puzzle difficulty and configuration.
pub trait Puzzle {
    /// The order of the puzzle.
    fn order(&self) -> u8;
}
