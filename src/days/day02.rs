//! ## --- Day 2: Inventory Management System ---
//!
//! Straightforward solutions, but under 1ms for both parts.
//!

use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn count_freqs(s: &str) -> HashMap<char, u32> {
    let mut freqs = HashMap::new();

    for ch in s.chars() {
        *freqs.entry(ch).or_insert(0) += 1;
    }

    freqs
}

/// What is the checksum for your list of box IDs?
pub fn part1(input: &[String]) -> u32 {
    let checksum_components =
        input.iter().map(|s| count_freqs(s.as_str())).fold((0, 0), |acc, x| {
            (
                acc.0 + x.values().any(|count| *count == 2) as u32,
                acc.1 + x.values().any(|count| *count == 3) as u32,
            )
        });

    checksum_components.0 * checksum_components.1
}

fn is_correct_ids(box1: &str, box2: &str) -> bool {
    let mut idx = 0;
    let mut diff = 0;
    loop {
        if box1.as_bytes()[idx] != box2.as_bytes()[idx] {
            diff += 1;
            if diff > 1 {
                return false;
            }
        }
        idx += 1;
        if idx == box1.len() {
            break;
        }
    }
    if diff == 0 {
        return false;
    }
    true
}

fn common_chars(box1: &str, box2: &str) -> String {
    box1.chars().zip(box2.chars()).filter_map(|(c1, c2)| (c1 == c2).then_some(c1)).collect()
}

/// What letters are common between the two correct box IDs?
pub fn part2(input: &[String]) -> String {
    for box1 in input {
        for box2 in input {
            if is_correct_ids(box1, box2) {
                return common_chars(box1, box2);
            }
        }
    }
    unreachable!();
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
