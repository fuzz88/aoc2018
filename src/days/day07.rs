//! ## --- Day 7: The Sum of Its Parts ---
//!

pub fn parse(input: &str) -> Vec<i32> {
    vec![]
}

pub fn part1(input: &[i32]) -> i32 {
    0
}

pub fn part2(input: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod test {
    
    use super::*;

    #[test]
    fn sample1() {
        let input = "/
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";
        let input_data = parse(&input);
        assert!(true);
    }
}
