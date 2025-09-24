//! Utilities to work with points on the plane.

use crate::utils::parse::*;
use std::ops::Add;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Velocity {
    pub dx: i32,
    pub dy: i32,
}

/// Coordinates of the point.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    /// Manhattan distance from point to point.
    pub fn manhattan_distance(self, other: Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

/// Minimal bounding box that contains input list of points.
#[derive(Debug)]
pub struct BoundingBox {
    pub top: i32,
    pub left: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<&[Point]> for BoundingBox {
    /// Calculates the bounding for the list of points.
    fn from(points: &[Point]) -> Self {
        let mut top = i32::MAX; // min_y
        let mut left = i32::MAX; // min_x
        let mut right = i32::MIN; // max_x
        let mut bottom = i32::MIN; // max_y

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
    /// Checks if the point is inside the `BoundingBox`.
    pub fn contains(&self, point: Point) -> bool {
        point.x > self.left && point.x < self.right && point.y > self.top && point.y < self.bottom
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut iter = value.iter_signed();

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        Point { x, y }
    }
}
