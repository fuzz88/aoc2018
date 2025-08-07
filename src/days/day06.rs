//! ## --- Day 6: Chronal Coordinates ---
//!

use crate::utils::parse::*;

/// (0, 0) at top left.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: u32,
    y: u32,
}

/// Global bounding box contains (bounds) all the points.
#[derive(Debug)]
pub struct BoundingBox {
    top: u32,
    left: u32,
    right: u32,
    bottom: u32,
}

impl From<&[Point]> for BoundingBox {
    /// Calculates the boundings for the list of points.
    fn from(points: &[Point]) -> Self {
        let mut top = u32::MAX; // min_y
        let mut left = u32::MAX; // min_x
        let mut right = u32::MIN; // max_x
        let mut bottom = u32::MIN; // max_y

        for point in points {
            if point.x > right {
                right = point.x + 1;
            }
            if point.x < left {
                assert!(point.x > 0);
                left = point.x - 1;
            }
            if point.y > bottom {
                bottom = point.y + 1;
            }
            if point.y < top {
                assert!(point.y > 0);
                top = point.y - 1;
            }
        }

        BoundingBox { top, left, right, bottom }
    }
}

impl BoundingBox {
    /// Checks if the point is inside the BoundingBox.
    fn contains(&self, point: &Point) -> bool {
        point.x > self.left && point.x < self.right && point.y > self.top && point.y < self.bottom
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut iter = value.iter_unsigned();

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Point { x, y }
    }
}

pub fn parse(input: &str) -> Vec<Point> {
    input.lines().map(Point::from).collect()
}

pub fn part1(input: &[Point]) -> usize {
    dbg!(BoundingBox::from(input));
    0
}

pub fn part2(input: &[Point]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let input_data = parse(input);
        assert_eq!(input_data.len(), 6);

        assert_eq!(part1(&input_data), 17);
    }

    #[test]
    fn bounding_box_contains() {
        let input = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
        let input_data = parse(input);
        assert_eq!(input_data.len(), 6);

        let bounding_box = BoundingBox::from(input_data.as_slice());
        assert!(bounding_box.contains(&Point { x: 3, y: 4 }));
        assert!(bounding_box.contains(&Point { x: 1, y: 1 }));
        assert!(bounding_box.contains(&Point { x: 8, y: 9 }));
        assert!(!bounding_box.contains(&Point { x: 0, y: 0 }));
        assert!(!bounding_box.contains(&Point { x: 9, y: 9 }));
    }
}
