//! ## --- Day 1: Chronal Calibration ---
//! Seems simple.
//!
//! Part2: also tried binary search approach to check if frequency was already met, but that is
//! faster only for very short inputs with very short cycles.
//!
//! For example, for a `sample1` binary search is almost 10x faster.

use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse::<i32>().unwrap()).collect()
}

/// Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[allow(dead_code)]
fn part2_bs(input: &[i32]) -> i32 {
    let mut numbers = vec![];
    let mut current_frequency = 0;

    for diff in input.iter().cycle() {
        current_frequency += diff;
        if let Err(idx) = numbers.binary_search(&current_frequency) {
            numbers.insert(idx, current_frequency);
        } else {
            return current_frequency;
        }
    }

    unreachable!("invalid input: frequency was not encountered twice");
}

/// What is the first frequency your device reaches twice?
pub fn part2(input: &[i32]) -> i32 {
    let mut numbers = HashSet::new();
    let mut current_frequency = 0;

    for diff in input.iter().cycle() {
        current_frequency += diff;
        if numbers.contains(&current_frequency) {
            return current_frequency;
        }
        numbers.insert(current_frequency);
    }

    unreachable!("invalid input: frequency was not encountered twice");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn sample1() {
        // run as `cargo test -- --nocapture` to see time measurments
        let sample = "+1\n-2\n+3\n+1";
        let input_data = parse(sample);

        assert_eq!(input_data.len(), 4);
        assert_eq!(part1(&input_data), 3);

        let instant = Instant::now();
        assert_eq!(part2(&input_data), 2);
        println!("{}", instant.elapsed().as_nanos());

        let instant = Instant::now();
        assert_eq!(part2_bs(&input_data), 2);
        println!("{}", instant.elapsed().as_nanos());
    }
}
