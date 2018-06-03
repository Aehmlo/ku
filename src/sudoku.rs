use sol::{score, solve, Error as SolveError};
use Puzzle;
use Score;
use Solve;
use DIMENSIONS;

use std::{
    fmt, ops::{Index, IndexMut}, str::FromStr,
};

/// Represents a single sudoku "square."
///
/// The quantum of the sudoku.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Element(pub u8);

/// A subdivision of the main sudoku; the smallest grouping to which rules are applied.
#[derive(Clone, Debug)]
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
    ///
    /// ### Dimensionality
    /// In *n* dimensions, `n - 1` bands apply to each element.
    /// Each is linearly independent from the others and from the relevant stack.
    Band(Vec<Option<Element>>),
}

impl Group {
    /// Whether a group is valid (contains no errors).
    ///
    /// A group is considered valid if it contains only unique elements
    /// (ignoring empty elements).
    pub fn is_valid(&self) -> bool {
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
    pub fn is_complete(&self) -> bool {
        let elements = self.elements();
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
    pub fn elements(&self) -> Vec<Option<Element>> {
        use self::Group::*;
        match self {
            Box(elements) | Stack(elements) | Band(elements) => elements.clone(),
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Group::Box(vec![])
    }
}

#[derive(Clone, Debug, PartialEq)]
/// A (partial) grid of [elements](struct.Element.html).
pub struct Sudoku {
    /// The [order](trait.Puzzle.html#method.order) of this sudoku.
    pub order: u8,
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

    /// Creates a point with the given x-coordinate and all other coordinates zero.
    pub fn with_x(value: u8) -> Self {
        let mut point = [0; DIMENSIONS];
        point[0] = value;
        Point(point)
    }

    /// Creates a point with the given y-coordinate and all other coordinates zero.
    pub fn with_y(value: u8) -> Self {
        let mut point = [0; DIMENSIONS];
        point[1] = value;
        Point(point)
    }

    #[cfg(feature = "3D")]
    /// Creates a point with the given z-coordinate and all other coordinates zero.
    pub fn with_z(value: u8) -> Self {
        let mut point = [0; DIMENSIONS];
        point[2] = value;
        Point(point)
    }

    /// The point with all coordinates identically zero.
    pub fn origin() -> Self {
        Point([0; DIMENSIONS])
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(")?;
        for i in 0..DIMENSIONS - 1 {
            write!(f, "{}, ", self[i])?;
        }
        write!(f, "{})", self[DIMENSIONS - 1])
    }
}

/// Represents an *n*-dimensional grid of values, indexable via [`Point`](struct.Point.html).
pub trait Grid: Index<Point> {
    /// Returns all points in the grid.
    ///
    /// Useful for enumeration with `Iterator::zip`.
    fn points(&self) -> Vec<Point>;
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
            elements: vec![None; (order as usize).pow(2 + DIMENSIONS as u32)],
        }
    }

    /// Returns whether the puzzle is completely full of values.
    pub fn is_complete(&self) -> bool {
        for point in self.points() {
            if self[point].is_none() {
                return false;
            }
        }
        true
    }

    /// Returns the relevant groups for checking a given element in the grid.
    ///
    /// The number of groups is always equal to the number of dimensions plus
    /// one.
    pub fn groups(&self, pos: Point) -> [Group; DIMENSIONS + 1] {
        for i in 0..DIMENSIONS {
            assert!(pos[i] < self.order.pow(2));
        }
        let top_left = pos.snap(self.order);
        let order = self.order as i32;
        let points = self.points();
        let b = points
            .iter()
            .zip(self.elements.iter())
            .filter(|(index, _)| {
                let y = index[1];
                let x = index[0];
                let dy = y as i32 - top_left[1] as i32;
                let dx = x as i32 - top_left[0] as i32;
                if dy < 0 || dx < 0 || dy >= order || dx >= order {
                    return false;
                }
                true
            })
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();
        let b = Group::Box(b);

        let s = points
            .iter()
            .zip(self.elements.iter())
            .filter(|(index, _)| {
                if index[0] != pos[0] {
                    return false;
                }
                for i in 2..DIMENSIONS {
                    if index[i] != pos[i] {
                        return false;
                    }
                }
                true
            })
            .map(|(_, v)| *v)
            .collect::<Vec<_>>();
        let s = Group::Stack(s);
        let bands = (1..DIMENSIONS)
            .map(|i| {
                // The variant dimension
                let dimension = i - 1;
                points
                    .iter()
                    .zip(self.elements.iter())
                    .filter(|(index, _)| {
                        for j in 0..DIMENSIONS {
                            if j == dimension {
                                continue;
                            }
                            if pos[j] != index[j] {
                                return false;
                            }
                        }
                        true
                    })
                    .map(|(_, v)| *v)
                    .collect()
            })
            .map(|v| Group::Band(v))
            .collect::<Vec<_>>();
        let mut g = bands;
        g.insert(0, s);
        g.insert(0, b);
        // Here be dragons (not really, but update this when 1.27 gets stabilized)
        clone_into_array(&g[..=DIMENSIONS])
    }

    /// Places the specified value (or lack thereof) at the specified index,
    /// modifying in-place.
    pub fn substitute(&mut self, index: Point, value: Option<Element>) {
        self.elements[index.fold(self.order)] = value;
    }
}

impl Grid for Sudoku {
    fn points(&self) -> Vec<Point> {
        (0..(self.order as usize).pow(2 + DIMENSIONS as u32))
            .map(|p| Point::unfold(p, self.order))
            .collect()
    }
}

// https://stackoverflow.com/a/37682288
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
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
        solve(self)
    }
}

impl Score for Sudoku {
    fn score(&self) -> Option<usize> {
        score(self)
    }
}

#[cfg(feature = "2D")]
impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let order = self.order;
        let axis = order.pow(2);
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

// TODO((#7): Higher dimensions
#[cfg(feature = "2D")]
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
    use DIMENSIONS;

    // TODO(#9): Procedural macro-ify these tests
    // TODO(#8): Implement positive tests for Sudoku::groups
    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups(Point::with_x(9));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_3() {
        let sudoku = Sudoku::new(3);
        let _ = sudoku.groups(Point::with_y(9));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_x_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups(Point::with_x(16));
    }

    #[test]
    #[should_panic]
    fn test_sudoku_groups_index_y_4() {
        let sudoku = Sudoku::new(4);
        let _ = sudoku.groups(Point::with_y(16));
    }

    #[test]
    fn test_sudoku_groups_length_3_2d() {
        let sudoku = Sudoku::new(3);
        let groups = sudoku.groups(Point::origin());
        assert_eq!(groups[0].elements().len(), 3_usize.pow(DIMENSIONS as u32));
        assert_eq!(groups[1].elements().len(), 9);
        assert_eq!(groups[2].elements().len(), 9);
    }

    #[test]
    fn test_sudoku_groups_length_4_2d() {
        let sudoku = Sudoku::new(4);
        let groups = sudoku.groups(Point::origin());
        assert_eq!(groups[0].elements().len(), 4_usize.pow(DIMENSIONS as u32));
        assert_eq!(groups[1].elements().len(), 16);
        assert_eq!(groups[2].elements().len(), 16);
    }

    #[test]
    fn test_sudoku_new() {
        for order in 2..10usize {
            let sudoku = Sudoku::new(order as u8);
            assert_eq!(sudoku.elements.capacity(), order.pow(2 + DIMENSIONS as u32));
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

    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
    fn test_point_compose() {
        for i in 0..9 {
            for j in 0..9 {
                let point = Point([i, j]);
                assert_eq!(point, Point::unfold(point.fold(3), 3));
            }
        }
        for i in 0..16 {
            for j in 0..16 {
                let point = Point([i, j]);
                assert_eq!(point, Point::unfold(point.fold(4), 4));
            }
        }
    }

    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
    fn test_point_index() {
        for i in 0..9 {
            for j in 0..9 {
                let point = Point([i, j]);
                assert_eq!(point.0[0], point[0]);
            }
        }
    }

    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
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

    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
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
        for i in 0..16 {
            for j in 0..16 {
                let x = match i {
                    0...3 => 0,
                    4...7 => 4,
                    8...11 => 8,
                    12...15 => 12,
                    _ => unreachable!(),
                };
                let y = match j {
                    0...3 => 0,
                    4...7 => 4,
                    8...11 => 8,
                    12...15 => 12,
                    _ => unreachable!(),
                };
                let point = Point([i, j]);
                assert_eq!(point.snap(4), Point([x, y]));
            }
        }
    }
    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
    fn test_sudoku_from_str() {
        let possible = [
            include_str!("../tests/sudokus/solvable/2D-O3.txt"),
            include_str!("../tests/sudokus/solvable/2D-O4.txt"),
        ];
        for s in possible.iter() {
            let puzzle = s.parse::<Sudoku>();
            assert!(puzzle.is_ok());
        }
        let impossible = [
            include_str!("../tests/sudokus/invalid/2D-O3.txt"),
            include_str!("../tests/sudokus/invalid/2D-O4.txt"),
        ];
        for s in impossible.iter() {
            let puzzle = s.parse::<Sudoku>();
            assert!(puzzle.is_err());
        }
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[cfg_attr(feature = "2D", test)]
    #[cfg(feature = "2D")]
    fn test_sudoku_from_str_parse_compose() {
        let s = include_str!("../tests/sudokus/solvable/2D-O3.txt");
        let puzzle = s.parse::<Sudoku>();
        assert!(puzzle.is_ok());
        assert_eq!(&format!("{}", puzzle.unwrap()), s);
    }
}
