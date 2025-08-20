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
    let mut dependants: Vec<u8> = input.iter().flat_map(|(k, v)| v).copied().collect();
    dependants.sort_unstable();
    dependants.dedup();
    let mut starters = vec![];
    
    for step in input.keys() {
        if let Err(_) = dependants.binary_search(step) {
            println!("{}", *step);
            starters.push(*step);
        }
    }

    starters.sort_unstable();

    println!("{:?}", starters);
    println!("{:?}", dependants);

    let mut order = String::new();
    let mut seen = HashSet::<u8>::new();

    fn dfs(step: u8, input: &Steps, seen: &mut HashSet<u8>, order: &mut String) {
        seen.insert(step);
        if let Some(deps) = input.get(&step) {
            for dep in deps.iter() {
                if !seen.contains(dep) {
                    dfs(*dep, input, seen, order);
                }
            }
        }
        order.push(step as char);
    }

   let starters = [65]; 

    for starter in starters {
        let mut sub_order = String::new();
        dfs(starter, input, &mut seen, &mut sub_order);
        let s: String = sub_order.chars().rev().collect();
        println!("{}", s);
        println!("{}", s.len());
        order.push_str(&s);
    }

    order
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
