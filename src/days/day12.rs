//! ## --- Day 12: Subterranean Sustainability ---
//!

use std::collections::HashMap;

type Pot = u8;
type Note = Vec<u8>;
type Rules = HashMap<Note, Pot>;
type State = Vec<u8>;
type Input = (State, Rules);

pub fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let initial_state = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => unreachable!(),
        })
        .collect();

    let mut rules = Rules::new();

    for line in lines.skip(1) {
        let components: Vec<&str> = line.split(" => ").collect();

        let note = components[0]
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            })
            .collect();

        let pot = match components[1].chars().next().unwrap() {
            '#' => 1,
            '.' => 0,
            _ => unreachable!(),
        };

        rules.insert(note, pot);
    }

    (initial_state, rules)
}

pub fn part1(input: &Input) -> i32 {
    0
}

pub fn part2(input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

        let input_data = parse(input);
        println!("{:?}", input_data);
    }
}
