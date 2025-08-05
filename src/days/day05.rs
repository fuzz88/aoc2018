//! ## --- Day 5: Alchemical Reduction ---
//!
//! Nice. Came up with rotations and `VecDeque`, remembering [[Day 19, 2016]](<https://github.com/fuzz88/aoc2016/blob/ebc64f89adbdbf825d8ed03081cade4404c0a2e8/day19/p1/main.rs#L19>)
//!
//! My original solution for [Day19, 2016] was slow, about 1,5min as far as I remember.
//! I realised, that slow part was inserting and deleting elements in Vec, which is O(n).
//! Before I started to optimize and research how to avoid those inserts and deletions I asked chatgpt, because I was really in doubt.
//! "Is there some formulae as in part 1?", which I derived by cranking through some numbers by hand,
//! and it said "No, there is no formulae, but you can use `VecDeque` to speed up things". 🫠 Oh,
//! damn.

use std::collections::VecDeque;
use std::sync::mpsc::channel;
use std::thread;

pub fn parse(input: &str) -> &[u8] {
    input.as_bytes()
}

/// How many units remain after fully reacting the polymer you scanned?
pub fn part1(input: &[u8]) -> usize {
    let mut input = VecDeque::from(input.to_owned());

    loop {
        if input.len() > 1 && input[0].abs_diff(input[1]) == 32 {
            input.pop_front();
            input.pop_front();
            input.rotate_right(1);
        } else {
            input.rotate_left(1);
            if input[0] == 10 {
                break;
            }
        }
    }

    input.len() - 1
}

/// What is the length of the shortest polymer you can produce
/// by removing all units of exactly one type and fully reacting the result?
pub fn part2(input: &[u8]) -> usize {
    let mut min_len = usize::MAX;

    let (tx, rx) = channel();

    for ch in b'a'..=b'z' {
        let mut input = VecDeque::from(input.to_owned());
        let tx = tx.clone();

        thread::spawn(move || {
            loop {
                if input[0] == ch || input[0] == ch - 32 {
                    input.pop_front();
                    input.rotate_right(1);
                    continue;
                }
                if input.len() > 1 && input[0].abs_diff(input[1]) == 32 {
                    input.pop_front();
                    input.pop_front();
                    input.rotate_right(1);
                } else {
                    input.rotate_left(1);
                    if input[0] == 10 {
                        break;
                    }
                }
            }
            tx.send(input.len() - 1).unwrap();
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
        let input = "dabAcCaCBAcCcaDA\n";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 10);
        assert_eq!(part2(input_data), 4);

        let input = "aA\n";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 0);
        let input = "abBA\n";
        let input_data = parse(input);
        assert_eq!(part1(input_data), 0);
    }
}
