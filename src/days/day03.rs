//! ## --- Day 3: No Matter How You Slice It ---
//!
//! In Part1: iterating each square inch of all claims, using two `HashSets`:
//! - first one for checking if square inch is overlapped by another square inch.
//! - second one to avoid counting duplicate overlaps.
//!
//! In Part2: as we don't need to count number of overlaps, simply check if pairs of claims (rectangles) overlap in 2D, searching for the one claim  which doesn't overlap with any other.

use crate::utils::parse::*;
use std::collections::HashSet;

/// Claim for fabric: 2D rectangle with id.
pub struct Claim {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl FromIterator<u32> for Claim {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = u32>,
    {
        let mut iter = iter.into_iter();

        Claim {
            id: iter.next().unwrap(),
            left: iter.next().unwrap(),
            top: iter.next().unwrap(),
            width: iter.next().unwrap(),
            height: iter.next().unwrap(),
        }
    }
}

impl Claim {
    /// Checks if this rectangle overlaps with another one.
    fn overlaps(&self, other: &Claim) -> bool {
        let self_right = self.left + self.width - 1;
        let self_bottom = self.top + self.height - 1;

        let other_right = other.left + other.width - 1;
        let other_bottom = other.top + other.height - 1;

        if self.left > other_right || self_right < other.left {
            return false;
        }

        if self.top > other_bottom || self_bottom < other.top {
            return false;
        }

        true
    }
}

pub fn parse(input: &str) -> Vec<Claim> {
    let as_claim = |line: &str| line.iter_signed().collect();
    input.lines().map(as_claim).collect()
}

/// How many square inches of fabric are within two or more claims?
pub fn part1(input: &[Claim]) -> u32 {
    let mut all_points = HashSet::new();
    let mut overlapped = HashSet::new();

    for claim in input {
        for x in claim.left..claim.left + claim.width {
            for y in claim.top..claim.top + claim.height {
                if all_points.contains(&(x, y)) {
                    overlapped.insert((x, y));
                } else {
                    all_points.insert((x, y));
                }
            }
        }
    }

    overlapped.len() as u32
}

/// What is the ID of the only claim that doesn't overlap?
pub fn part2(input: &[Claim]) -> u32 {
    for i in 0..input.len() {
        let mut overlapped = false;

        for j in 0..input.len() {
            if i != j && input[i].overlaps(&input[j]) {
                overlapped = true;
                break;
            }
        }
        if !overlapped {
            return input[i].id;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let input_data = parse(input);

        assert_eq!(input_data.len(), 3);

        assert_eq!(part1(&input_data), 4);
        assert_eq!(part2(&input_data), 3);
    }

    #[test]
    fn has_and_overlaps() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let input_data = parse(input);

        assert_eq!(input_data.len(), 3);

        assert!(input_data[0].overlaps(&input_data[1]));
        assert!(!input_data[0].overlaps(&input_data[2]));
        assert!(!input_data[1].overlaps(&input_data[2]));
    }
}
