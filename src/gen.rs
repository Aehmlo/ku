use rand::{thread_rng, Rng};
use sol::PossibilityMap;
use Difficulty;
use Element;
use Grid;
use Score;
use Sudoku;

/// The maximum number of times the hardening algorithm will try to make a harder puzzle in a
/// single pass.
const MAX_HARDEN_ITERATIONS: u8 = 20;

/// Trait to generate a puzzle.
///
/// Requires that the puzzle be solvable (to ensure the desired difficulty is
/// attained).
pub trait Generate: Score + Sized {
    /// Generates a puzzle of the desired order and difficulty.
    fn generate(order: u8, difficulty: Difficulty) -> Self;
}

fn take_random<T>(values: &mut Vec<T>) -> Option<T> {
    let mut rng = thread_rng();
    let mut indices = (0..values.len()).collect::<Vec<_>>();
    rng.shuffle(&mut indices);
    indices.get(0).map(|index| values.remove(*index))
}

fn recurse(puzzle: Sudoku) -> Option<Sudoku> {
    let map: PossibilityMap = puzzle.clone().into();
    match map.next() {
        (None, _) => {
            if puzzle.is_complete() {
                Some(puzzle)
            } else {
                None
            }
        }
        (Some(index), Some(set)) => {
            let mut possibilities = (1..=(puzzle.order as usize).pow(2))
                .filter(|v| set.contains(*v))
                .collect::<Vec<_>>();
            while let Some(candidate) = take_random(&mut possibilities) {
                let mut puzzle = puzzle.clone();
                puzzle.substitute(index, Some(Element(candidate as u8)));
                let solution = recurse(puzzle);
                if solution.is_some() {
                    return solution;
                }
            }
            None
        }
        _ => unreachable!(),
    }
}

/// Creates a randomized sudoku grid of the specified order.
fn grid(order: u8) -> Option<Sudoku> {
    let mut rng = thread_rng();
    let mut puzzle = Sudoku::new(order);
    // TODO(#14): Revisit this block when NLL lands.
    {
        let mut first_box = (1..=order.pow(2))
            .map(|v| Some(Element(v)))
            .collect::<Vec<_>>();
        rng.shuffle(&mut first_box);
        let order = order as usize;
        let axis = order.pow(2);
        for i in 0..axis {
            let index = i / order * axis + i % order;
            puzzle.elements[index] = first_box[i];
        }
        // TODO(#13): Reduce the number of cells that are filled with backtracking.
        // The rest
        recurse(puzzle)
    }
}

/// Makes the sudoku harder to the desired level, modifying it in-place.
///
/// # Notes
/// No validation is performed on the passed puzzle.
fn harden(mut sudoku: &mut Sudoku, target: Difficulty) -> Result<(), ()> {
    let current = sudoku.score().unwrap();
    let mut points = sudoku.points();
    for _ in 0..MAX_HARDEN_ITERATIONS {
        if let (Some(one), Some(two)) = (take_random(&mut points), take_random(&mut points)) {
            let (one, two) = (one.fold(sudoku.order), two.fold(sudoku.order));
            let mut puzzle = sudoku.clone();
            // Faster than substituting twice.
            puzzle.elements[one] = None;
            puzzle.elements[two] = None;
            match puzzle.score() {
                Some(score) => {
                    if score > current {
                        let difficulty: Difficulty = score.into();
                        if difficulty > target {
                            // We overshot the target difficulty
                            continue;
                        }
                        sudoku.elements[one] = None;
                        sudoku.elements[two] = None;
                        return if difficulty == target {
                            Ok(())
                        } else {
                            harden(&mut sudoku, target)
                        };
                    }
                }
                _ => {}
            }
        }
    }
    Err(())
}

impl Generate for Sudoku {
    fn generate(order: u8, difficulty: Difficulty) -> Self {
        let mut puzzle = grid(order).unwrap();
        let _ = harden(&mut puzzle, difficulty);
        puzzle
    }
}

#[cfg(test)]
mod tests {
    use gen;
    use Solve;
    #[cfg_attr(feature = "2D", test)]
    fn test_grid() {
        let grid = gen::grid(3);
        let grid = grid.unwrap();
        assert!(grid.is_complete());
        assert!(grid.is_uniquely_solvable());
    }
}
