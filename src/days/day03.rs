//! --- Day 3: No Matter How You Slice It ---
//!
//!

use crate::utils::parse::*;

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

pub fn parse(input: &str) -> Vec<Claim> {
    let as_claim = |line: &str| line.iter_signed().collect();
    input.lines().map(as_claim).collect()
}

pub fn part1(input: &[Claim]) -> u32 {
    0
}

pub fn part2(input: &[Claim]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let input_data = parse(&input);

        assert_eq!(input_data.len(), 3);
        assert_eq!(part1(&input_data), 4);
    }
}
