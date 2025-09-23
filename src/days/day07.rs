//! ## --- Day 7: The Sum of Its Parts ---
//!
//! I struggled with this problem couple of month ago. Very first idea that I came up with was to use
//! some kind of topological sorting. That appeared to be right.
//! I studied wikipedia article on the topic, and tried to implement pseudocode given there, but failed miserably.
//! Then I looked up the solution in the repo nearby, found notes on topological sorting,
//! and had looked up the usage of `BTreeMap`, then decided to take a pause from solving those
//! problems, because the last thing I wanted to do was to adapt freshly reminded code.
//!
//! This is my own code and I am kind of proud of myself for now. It came out much simplier than I remebered.

use std::collections::{BTreeMap, HashMap};

type Steps = HashMap<u8, Step>;

#[derive(Default, Clone)]
pub struct Step {
    pub children: Vec<u8>,
    pub deps_count: usize,
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

    let mut all = input.clone();
    let mut ready: BTreeMap<u8, Step> = all
        .iter()
        .filter(|&(_, step)| step.deps_count == 0)
        .map(|(idx, step)| (*idx, step.clone()))
        .collect();

    while let Some((idx, step)) = ready.pop_first() {
        result.push(idx as char);
        for child in &step.children {
            let step = all.get_mut(child).unwrap();
            step.deps_count -= 1;
            if step.deps_count == 0 {
                ready.insert(*child, step.clone());
            }
        }
    }

    result
}

fn part2_testable(input: &Steps, workers_num: u8, duration_base: u32) -> u32 {
    let mut duration: u32 = 0;
    let mut workers: u8 = workers_num;

    let mut all = input.clone();
    let mut ready: BTreeMap<u8, Step> = all
        .iter()
        .filter(|&(_, step)| step.deps_count == 0)
        .map(|(idx, step)| (*idx, step.clone()))
        .collect();

    let mut processing = vec![];

    loop {
        while workers > 0
            && let Some((step_idx, step)) = ready.pop_first()
        {
            processing.push((step_idx, step, duration));
            workers -= 1;
        }

        if processing.is_empty() {
            break;
        }

        duration += 1;

        for (processing_idx, (step_idx, step, started)) in processing.clone().iter().enumerate() {
            if duration - started == duration_base + *step_idx as u32 - 64 {
                processing.remove(processing_idx);
                if workers < workers_num {
                    workers += 1;
                }
                for child in &step.children {
                    let step = all.get_mut(child).unwrap();
                    step.deps_count -= 1;
                    if step.deps_count == 0 {
                        ready.insert(*child, step.clone());
                    }
                }
            }
        }
    }

    duration
}

pub fn part2(input: &Steps) -> u32 {
    part2_testable(input, 5, 60)
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
        let input_data = parse(input);
        assert_eq!(part1(&input_data), "CABDFE");
        assert_eq!(part2_testable(&input_data, 2, 0), 15);
    }
}
