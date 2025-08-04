//! ## --- Day 5: Alchemical Reduction ---
//!
//! Not the best solutions so far, but came up with this at the moment.

use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::thread;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

/// How many units remain after fully reacting the polymer you scanned?
pub fn part1(input: &[u8]) -> usize {
    let mut input = VecDeque::from(input.trim_ascii_end().to_owned());
    let mut rotations = 0;

    loop {
        // println!("{:?}", &input);
        // println!("{}", rotations);
        if input[0].abs_diff(input[1]) == 32 {
            input.pop_front();
            input.pop_front();
            // println!("pop");

            if input.is_empty() {
                break;
            }
            input.rotate_right(1);
            rotations = 0;
        } else {
            input.rotate_left(1);
            rotations += 1;
            if rotations == input.len() {
                break;
            }
        }
    }

    input.len()
}

/// What is the length of the shortest polymer you can produce by removing all units of exactly one type and fully reacting the result?
pub fn part2(input: &[u8]) -> usize {
    let mut min_len = usize::MAX;

    let (tx, rx) = channel();

    for ch in b'a'..=b'z' {
        let mut input = input
            .trim_ascii_end()
            .iter()
            .filter(|el| **el != ch && **el != ch - 32)
            .copied()
            .collect::<Vec<_>>();

        let tx = tx.clone();

        thread::spawn(move || {
            let mut idx = 0;
            while !input.is_empty() && idx != input.len() - 1 {
                if input[idx].abs_diff(input[idx + 1]) == 32 {
                    input.remove(idx);
                    input.remove(idx);
                    idx = 0;
                } else {
                    idx += 1;
                }
            }
            tx.send(input.len()).unwrap();
        });
    }
    drop(tx);

    while let Ok(len) = rx.recv() {
        if min_len > len {
            min_len = len;
        }
    }

    min_len
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "dabAcCaCBAcCcaDA";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 10);
        assert_eq!(part2(input_data), 4);

        let input = "aA";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 0);
        let input = "abBA";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 0);
    }
}
