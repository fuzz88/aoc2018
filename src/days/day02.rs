//! ## --- Day 2: Inventory Management System ---
//!

use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn freqs(s: &str) -> HashMap<char, u32> {
    let mut freqs = HashMap::new();

    for ch in s.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }

    freqs
}

/// What is the checksum for your list of box IDs?
pub fn part1(input: &[String]) -> u32 {
    let checksum_components = input.iter().map(|s| freqs(s.as_str())).fold((0, 0), |acc, x| {
        (
            acc.0 + x.values().any(|count| *count == 2) as u32,
            acc.1 + x.values().any(|count| *count == 3) as u32,
        )
    });

    checksum_components.0 * checksum_components.1
}

/// What letters are common between the two correct box IDs?
pub fn part2(input: &[String]) -> String {
    0.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";
        let input_data = parse(input);

        assert_eq!(part1(&input_data), 12);
    }
}
