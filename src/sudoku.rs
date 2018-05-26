use sol::Error as SolveError;
use Generate;
use Puzzle;
use Score;
use Solve;

const DIMENSIONS: usize = 2; // We may allow changing this later.

/// Represents a single sudoku "square."
///
/// The quantum of the sudoku.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Element(u8);

/// A subdivision of the main sudoku; the smallest grouping to which rules are applied.
#[derive(Debug)]
pub enum Group {
    /// A square set of [elements](struct.Element.html).
    ///
    /// A subdivision of a [sudoku](struct.sudoku.html).
    ///
    /// ### Rule
    /// Each box may contain each element value only once.
    Box(Vec<Option<Element>>),
    /// A vertical set of [elements](struct.Element.html).
    ///
    /// A subdivision of a [sudoku](struct.sudoku.html).
    ///
    /// ### Rule
    /// Each stack may contain each element value only once.
    Stack(Vec<Option<Element>>),
    /// A horizontal set of [elements](struct.Element.html).
    ///
    /// A subdivision of a [sudoku](struct.sudoku.html).
    ///
    /// ### Rule
    /// Each band may contain each element value only once.
    Band(Vec<Option<Element>>),
}

impl Group {
    /// Whether a group is valid (contains no errors).
    ///
    /// A group is considered valid if it contains only unique elements (ignoring empty elements).
    fn is_valid(&self) -> bool {
        let elements = self.elements();
        let elements = elements.iter().filter(|e| e.is_some()).collect::<Vec<_>>();
        let len = elements.len();
        let mut elements = elements.into_iter().collect::<Vec<_>>();
        elements.sort();
        elements.dedup();
        elements.len() == len
    }
    /// Whether a group is complete.
    ///
    /// A group is considered complete if it contains every possible element value exactly once.
    fn is_complete(&self) -> bool {
        let elements = self.elements();
        let elements = elements.iter().filter(|e| e.is_some()).collect::<Vec<_>>();
        let len = elements.len();
        let mut elements = elements
            .into_iter()
            .filter(|e| e.is_some())
            .collect::<Vec<_>>();
        elements.sort();
        elements.dedup();
        elements.len() == len
    }
    /// Returns an owned copy of the group's constituent elements.
    fn elements(&self) -> Vec<Option<Element>> {
        use self::Group::*;
        match self {
            Box(elements) | Stack(elements) | Band(elements) => elements.clone(),
        }
    }
}

#[derive(Debug)]
/// A (partial) grid of [elements](struct.Element.html).
pub struct Sudoku {
    /// The [order](trait.Puzzle.html#method.order) of this sudoku.
    order: u8,
    /// The [elements](struct.Element.html) composing this sudoku.
    pub elements: Vec<Option<Element>>,
}

/// Specifies a sudoku element's location in space.
///
/// The point is fully specified in `DIMENSIONS` dimensions.
///
/// # Coordinate System
/// The coordinate system used in this library sets the origin in the top-left corner, with
/// increasing x to the right and increasing y downward.
///
/// Additional axes (if applicable) follow the right-hand rule.
pub type Point = [u8; DIMENSIONS];

impl Sudoku {
    /// Constructs a new sudoku of the specified order.
    ///
    /// This method reserves space in memory for the puzzle's elements.
    ///
    /// # Notes
    /// This method **does not** generate a valid, uniquely solvable sudoku.
    /// If you wish to generate such a sudoku (which you likely do), use
    /// [`Sudoku::generate`](#method.generate).
    fn new(order: u8) -> Self {
        Self {
            order,
            elements: Vec::with_capacity((order as usize).pow(4)),
        }
    }

    /// Returns the relevant groups for checking a given element in the grid.
    ///
    /// The number of groups is always equal to the number of dimensions plus one.
    pub fn groups(&self, pos: Point) -> [Group; DIMENSIONS + 1] {
        assert!(pos[0] < self.order.pow(2));
        assert!(pos[1] < self.order.pow(2));
        unimplemented!()
    }
}

impl Puzzle for Sudoku {
    fn order(&self) -> u8 {
        self.order
    }
}

impl Solve for Sudoku {
    fn solution(&self) -> Result<Self, SolveError> {
        unimplemented!() // TODO: Find sudoku solutions
    }
}

impl Score for Sudoku {
    fn score(&self) -> Option<u16> {
        unimplemented!()
    }
}

impl Generate for Sudoku {}

#[cfg(test)]
mod tests {
    use sudoku::{Element, Group, Sudoku};
    use Puzzle;

    // TODO: Procedural macro-ify these tests
    // TODO: Implement positive tests for Sudoku::groups
    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups([9, 0]);
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups([0, 9]);
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups([16, 0]);
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups([0, 16]);
    }

    #[test]
    fn test_sudoku_new() {
        for order in 2..10usize {
            let sudoku = Sudoku::new(order as u8);
            assert_eq!(sudoku.elements.capacity(), order.pow(4));
        }
    }

    #[test]
    fn test_group_is_valid() {
        let group = Group::Box(vec![]);
        assert!(group.is_valid());
        let group = Group::Box(vec![Some(Element(1)), Some(Element(1))]);
        assert!(!group.is_valid());
    }

    #[test]
    fn test_group_is_complete() {
        for vec in [vec![], vec![Some(Element(1)), Some(Element(2))]].into_iter() {
            let group = Group::Box(vec.clone());
            assert!(group.is_complete());
        }
        let group = Group::Box(vec![Some(Element(1)), Some(Element(1))]);
        assert!(!group.is_complete());
    }

    #[test]
    fn test_group_elements() {
        for vec in [vec![], vec![Some(Element(2)), Some(Element(6)), None]].into_iter() {
            let group = Group::Box(vec.clone());
            assert_eq!(&group.elements(), vec);
        }
    }

    #[test]
    fn test_sudoku_order() {
        for order in 1..10 {
            let sudoku = Sudoku::new(order);
            assert_eq!(sudoku.order(), order);
        }
    }
}
