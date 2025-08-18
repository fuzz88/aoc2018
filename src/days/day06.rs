//! ## --- Day 6: Chronal Coordinates ---
//!
//! I do not like this solution. TBD: why?

use crate::utils::parse::*;
use std::collections::{HashMap, HashSet};

/// Point(0, 0) is the top left.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(self, other: Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

/// Minimal bounding box that contains all the points.
#[derive(Debug)]
pub struct BoundingBox {
    top: i32,
    left: i32,
    right: i32,
    bottom: i32,
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
    fn contains(&self, point: Point) -> bool {
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

type Distances = HashMap<Point, HashMap<usize, u32>>;
type DistancesClosest = HashMap<Point, usize>;

/// `DFS` traverses closest area of the location given.
fn get_area_size(
    idx: usize,
    loc: Point,
    distances_closest: &DistancesClosest,
    bounding_box: &BoundingBox,
) -> usize {
    // let's collect points into area of the given location.
    let mut area = HashSet::<Point>::new();
    let mut to_process = vec![];

    area.insert(loc);
    to_process.push(loc);

    let is_closest = |p: Point| {
        if let Some(closest) = distances_closest.get(&p) {
            return *closest == idx;
        }
        false
    };

    while let Some(next_loc) = to_process.pop() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                if (dx == 0 && dy != 0) || (dy == 0 && dx != 0) {
                    let nx = next_loc.x + dx;
                    let ny = next_loc.y + dy;
                    let neighbour = Point { x: nx, y: ny };
                    // going out of the bounding box tells us that area is infinite,
                    if !bounding_box.contains(neighbour) {
                        // so we can break early.
                        return 0;
                    }
                    if !area.contains(&neighbour) && is_closest(neighbour) {
                        area.insert(neighbour);
                        to_process.push(neighbour);
                    }
                }
            }
        }
    }

    area.len()
}

/// For each point in the bounding box, this gives us distances to each input
/// location.
fn calculate_distances(points: &[Point], bounding_box: &BoundingBox) -> Distances {
    let mut distances = Distances::new();

    for col in bounding_box.left..=bounding_box.right {
        for row in bounding_box.top..=bounding_box.bottom {
            for (idx, loc) in points.iter().enumerate() {
                let current_point = Point { x: col, y: row };
                let point_distances = distances.entry(current_point).or_default();
                point_distances.insert(idx, loc.manhattan_distance(current_point));
            }
        }
    }

    distances
}

/// What is the size of the largest area that isn't infinite?
pub fn part1(input: &[Point]) -> usize {
    let bounding_box = BoundingBox::from(input);
    let distances = calculate_distances(input, &bounding_box);

    let mut distances_closest = DistancesClosest::new();

    for (point, dists) in &distances {
        for idx in 0..input.len() {
            let current_dist = dists.get(&idx).unwrap();
            if dists
                .iter()
                .all(|(i, x)| (*i != idx && x > current_dist) || (*i == idx && x == current_dist))
            {
                distances_closest.insert(*point, idx);
                break;
            }
        }
    }

    input
        .iter()
        .enumerate()
        .map(|(idx, &loc)| get_area_size(idx, loc, &distances_closest, &bounding_box))
        .max()
        .unwrap()
}

/// What is the size of the region containing all locations which have a total distance to all given coordinates of less than 10000?
pub fn part2(input: &[Point]) -> usize {
    let bounding_box = BoundingBox::from(input);
    let distances = calculate_distances(input, &bounding_box);

    distances.values().filter(|dists| dists.values().sum::<u32>() < 10000).count()
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

        // part2
        let bounding_box = BoundingBox::from(input_data.as_slice());
        let distances = calculate_distances(&input_data, &bounding_box);

        assert_eq!(distances.values().filter(|dists| dists.values().sum::<u32>() < 32).count(), 16);
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
        assert!(bounding_box.contains(Point { x: 3, y: 4 }));
        assert!(bounding_box.contains(Point { x: 1, y: 1 }));
        assert!(bounding_box.contains(Point { x: 8, y: 9 }));
        assert!(!bounding_box.contains(Point { x: 0, y: 0 }));
        assert!(!bounding_box.contains(Point { x: 9, y: 9 }));
    }
}
