use sol::Error as SolveError;
use Generate;
use Puzzle;
use Score;
use Solve;

use std::{
    fmt, ops::{Index, IndexMut}, str::FromStr,
};

const DIMENSIONS: usize = 2; // We may allow changing this later.

/// Represents a single sudoku "square."
///
/// The quantum of the sudoku.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Element(u8);

/// A subdivision of the main sudoku; the smallest grouping to which rules are
/// applied.
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
    /// A group is considered valid if it contains only unique elements
    /// (ignoring empty elements).
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
    /// A group is considered complete if it contains every possible element
    /// value exactly once.
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

#[derive(Debug, PartialEq)]
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
/// The coordinate system used in this library sets the origin in the top-left
/// corner, with increasing x to the right and increasing y downward.
///
/// Additional axes (if applicable) follow the right-hand rule.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point([u8; DIMENSIONS]);
impl Point {
    /// Compresses an *n*-dimensional point to a single coordinate.
    ///
    /// Inverse of [`Point::unfold`](#method.unfold).
    pub fn fold(&self, order: u8) -> usize {
        let axis = (order as usize).pow(2);
        let mut sum = 0;
        for i in 0..DIMENSIONS {
            sum += (self[i] as usize) * axis.pow(i as u32);
        }
        sum
    }

    /// Decompresses a single coordinate into an *n*-dimensional point.
    ///
    /// Inverse of [`Point::fold`](#method.fold).
    pub fn unfold(value: usize, order: u8) -> Self {
        let mut total = value;
        let axis = (order as usize).pow(2);
        let mut point = [0; DIMENSIONS];
        for i in 0..DIMENSIONS {
            let j = DIMENSIONS - i - 1;
            let discriminant = axis.pow(j as u32);
            let dim = total / discriminant;
            point[j] = dim as u8;
            total = total % discriminant;
        }
        Point(point)
    }

    /// Snaps a point to the grid (returns the upper-left corner of the box).
    pub fn snap(self, order: u8) -> Self {
        let mut point = self;
        for i in 0..DIMENSIONS {
            point[i] = self[i] - self[i] % order;
        }
        point
    }
}

impl Index<usize> for Point {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.0[index]
    }
}

impl Sudoku {
    /// Constructs a new sudoku of the specified order.
    ///
    /// This method reserves space in memory for the puzzle's elements.
    ///
    /// # Notes
    /// This method **does not** generate a valid, uniquely solvable sudoku.
    /// If you wish to generate such a sudoku (which you likely do), use
    /// [`Sudoku::generate`](#method.generate).
    pub fn new(order: u8) -> Self {
        Self {
            order,
            elements: Vec::with_capacity((order as usize).pow(4)),
        }
    }

    /// Returns the relevant groups for checking a given element in the grid.
    ///
    /// The number of groups is always equal to the number of dimensions plus
    /// one.
    pub fn groups(&self, pos: Point) -> [Group; DIMENSIONS + 1] {
        assert!(pos[0] < self.order.pow(2));
        assert!(pos[1] < self.order.pow(2));
        unimplemented!()
    }
}

impl Index<Point> for Sudoku {
    type Output = Option<Element>;
    fn index(&self, index: Point) -> &Self::Output {
        &self.elements[index.fold(self.order)]
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

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let order = self.order;
        let axis = order.pow(2);
        // TODO: Higher dimensions
        for y in 0..axis {
            for x in 0..axis {
                let element = self[Point([x, y])];
                match element {
                    Some(Element(value)) => {
                        write!(f, "{}", value)?;
                    }
                    None => {
                        write!(f, "_")?;
                    }
                }
                if x != axis - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

/// Represents a deserialization error.
#[derive(Clone, Copy, Debug)]
pub enum ParseError {
    /// Represents a grid with differing width and height.
    UnequalDimensions,
    /// Represents the presence of a value too large for the puzzle's dimensions.
    ///
    /// The associated values are the large value and its would-be location in the puzzle.
    LargeValue(u8, Point),
    /// Represents a grid with a non-perfect-square axial length.
    NonSquareAxis,
}

impl FromStr for Sudoku {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = s
            .split("\n")
            .map(|row| {
                row.split(" ")
                    .map(|cell| cell.parse().ok().map(|value| Element(value)))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let order = (rows.len() as f64).sqrt() as usize;
        println!("Order: {}", order);
        if rows.len() == order * order + 1 {
            let last = rows.pop().unwrap();
            if last.len() != 1 || last[0] != None {
                return Err(ParseError::NonSquareAxis);
            }
        }
        let axis = rows.len();
        if order * order != axis {
            return Err(ParseError::NonSquareAxis);
        }
        let mut elements = Vec::with_capacity(axis.pow(2));
        for j in 0..axis {
            let row = &rows[j];
            if row.len() != axis {
                return Err(ParseError::UnequalDimensions);
            }
            for i in 0..axis {
                if let Some(Element(value)) = row[i] {
                    if value > axis as u8 {
                        return Err(ParseError::LargeValue(value, Point([i as u8, j as u8])));
                    }
                }
                elements.push(row[i]);
            }
        }
        Ok(Sudoku {
            order: order as u8,
            elements,
        })
    }
}

#[cfg(test)]
mod tests {
    use sudoku::{Element, Group, Point, Sudoku};
    use Puzzle;

    // TODO: Procedural macro-ify these tests
    // TODO: Implement positive tests for Sudoku::groups
    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups(Point([9, 0]));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups(Point([0, 9]));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups(Point([16, 0]));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups(Point([0, 16]));
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

    #[test]
    fn test_point_compose() {
        for i in 0..9 {
            for j in 0..9 {
                let point = Point([i, j]);
                assert_eq!(point, Point::unfold(point.fold(3), 3));
            }
        }
    }

    #[test]
    fn test_point_index() {
        for i in 0..9 {
            for j in 0..9 {
                let point = Point([i, j]);
                assert_eq!(point.0[0], point[0]);
            }
        }
    }

    #[test]
    fn test_point_index_mut() {
        for i in 0..9 {
            for j in 0..9 {
                let mut point = Point([i, j]);
                point[0] = j;
                point[1] = i;
                assert_eq!(point[0], j);
                assert_eq!(point[1], i);
            }
        }
    }

    #[test]
    fn test_point_snap() {
        for i in 0..9 {
            for j in 0..9 {
                let x = match i {
                    0...2 => 0,
                    3...5 => 3,
                    6...8 => 6,
                    _ => unreachable!(),
                };
                let y = match j {
                    0...2 => 0,
                    3...5 => 3,
                    6...8 => 6,
                    _ => unreachable!(),
                };
                let point = Point([i, j]);
                assert_eq!(point.snap(3), Point([x, y]));
            }
        }
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_sudoku_from_str() {
        let possible = [
            "_ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n",
            "_ _ _ _ 2 _ _ _ _\n\
            _ _ _ _ _ 4 _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 9 _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ 7 _ _ _ _\n\
            _ _ _ _ _ _ _ 4 _\n\
            _ _ _ _ _ _ _ _ 1\n",
            "_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n",
            "_ _ _ _ 16 _ _ _ _ _ _ _ _ _ _ _\n\
            _ 1 _ _ _ 4 _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 9 _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ 7 _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 4 _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ 1 _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n",
        ];
        for s in possible.iter() {
            let puzzle = s.parse::<Sudoku>();
            assert!(puzzle.is_ok());
        }
        let impossible = [
            "_ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n",
            "_ _ _ _ 2 _ _ 10 _\n\
            _ _ _ _ _ 4 _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 9 _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ 7 _ _ _ _\n\
            _ _ _ _ _ _ _ 4 _\n\
            _ _ _ _ _ _ _ _ 1\n",
            "_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n",
            "_ _ _ _ 17 _ _ _ _ _ _ _ _ _ _ _\n\
            _ 1 _ _ _ 4 _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 9 _ _ _ 23 _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ 7 _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ 4 _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ 1 _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n\
            _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _\n",
        ];
        for s in impossible.iter() {
            let puzzle = s.parse::<Sudoku>();
            assert!(puzzle.is_err());
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_sudoku_from_str_parse_compose() {
        let s = "_ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n\
                 _ _ _ _ _ _ _ _ _\n";
        let puzzle = s.parse::<Sudoku>();
        assert!(puzzle.is_ok());
        assert_eq!(&format!("{}", puzzle.unwrap()), s);
    }
}
