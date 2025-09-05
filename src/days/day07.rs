//! ## --- Day 7: The Sum of Its Parts ---
//!

use std::collections::{HashMap, HashSet};

type Steps = HashMap<u8, Vec<u8>>;

pub fn parse(input: &str) -> Steps {
    let mut steps = Steps::new();

    for line in input.lines() {
        let bytes = line.as_bytes();
        steps.entry(bytes[5]).or_default().push(bytes[36]);
    }

    for (k, v) in &mut steps {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }

    steps
}


pub fn part1(input: &Steps) -> String {
    String::from("foo")
}

pub fn part2(input: &Steps) -> i32 {
    0
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn sample1() {
        let input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let input_data = parse(&input);
        assert_eq!(part1(&input_data), "CABDFE");
        assert!(true);
    }
}
