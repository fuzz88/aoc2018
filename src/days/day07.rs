//! ## --- Day 7: The Sum of Its Parts ---
//!
//! I struggled with this problem couple of month ago. Very first idea that I came up to was to use
//! some kind of topological sorting. That appeared to be right. I studied wikipedia article on the topic, and
//! tried to implement pseudocode given there, but failed miserably. Then I looked up the solution in the repo
//! nearby, found notes on topological sorting, and decided to take a pause from solving those problems.
//! I pretty much forget the looked up solution, and this is my own code and I am kind of proud
//! of myself for now. This came out much simplier than I remebered.

use std::collections::{BTreeMap, HashMap};

type Steps = HashMap<u8, Step>;

#[derive(Debug, Clone)]
pub struct Step {
    pub children: Vec<u8>,
    pub deps_count: usize,
}

impl Default for Step {
    fn default() -> Self {
        Step { children: vec![], deps_count: 0 }
    }
}

pub fn parse(input: &str) -> Steps {
    let mut steps = Steps::new();

    input.lines().for_each(|line| {
        let bytes = line.as_bytes();
        steps.entry(bytes[5]).or_default().children.push(bytes[36]);
        steps.entry(bytes[36]).or_default().deps_count += 1;
    });

    steps
}

pub fn part1(input: &Steps) -> String {
    let mut result = String::new();
    let mut ready = BTreeMap::<u8, Step>::new();
    let mut blocked = HashMap::<u8, Step>::new();

    for (&idx, step) in input {
        if step.deps_count == 0 {
            ready.insert(idx, step.clone());
        } else {
            blocked.insert(idx, step.clone());
        }
    }

    while let Some((idx, step)) = ready.pop_first() {
        result.push(idx as char);
        for child in step.children.iter() {
            let step = blocked.get_mut(child).unwrap();
            step.deps_count -= 1;
            if step.deps_count == 0 {
                ready.insert(*child, step.clone());
            }
        }
    }

    result
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
        println!("{:?}", input_data);
        assert_eq!(part1(&input_data), "CABDFE");
        assert!(true);
    }
}
