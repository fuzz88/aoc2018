//! ## --- Day 6: Chronal Coordinates ---
//!

use crate::utils::parse::*;

/// (0, 0) at top left.
pub struct Point {
    x: u32,
    y: u32,
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
}
