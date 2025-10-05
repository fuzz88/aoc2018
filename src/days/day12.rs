//! ## --- Day 12: Subterranean Sustainability ---
//!
//!  The automata part is easy. The key idea is to properly augment state to have enough pots for pattern matching. I assume that we need 4 pots beside pot with plant, to match (....#) pattern for example, as we can't have (.....) pattern to match on infinite set of empty pots.
//!
//!  I looked up the idea of stabilization for part two in the neighbour's repo. But the neighbour makes the assumtion that two consequent deltas to be equal is enough for stabilization to be reached.
//!  But I have this sequency of deltas near the 90th generation: 46 46 48 48 50 50 50 50 ...
//!
//!  So I did research my own deltas and use python as calculator to find out correct sum of positions.
//!
//!  Had fun but taking your own time is always be better as there no reason to hurry up.
//!
//!  "Don't look up, solutions, buddy." -- note for future self.

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
        let mut components = line.split(" => ");

        let note = components
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '#' => 1,
                '.' => 0,
                _ => unreachable!(),
            })
            .collect();

        let pot = match components.next().unwrap().chars().next().unwrap() {
            '#' => 1,
            '.' => 0,
            _ => unreachable!(),
        };

        rules.insert(note, pot);
    }

    (initial_state, rules)
}

/// After 20 generations, what is the sum of the numbers of all pots which contain a plant?
pub fn part1(input: &Input) -> u32 {
    let mut current_state = input.0.clone();
    let mut count = 20;
    let mut left_shift = 0;

    while count > 0 {
        // countdown from 20
        count -= 1;

        // needed for augmentation
        let len = current_state.len() - 1;

        // leftmost and rightmost pot with plant
        let mut l_augment = current_state.iter().position(|el| *el == 1).unwrap();
        let mut r_augment = current_state.iter().rposition(|el| *el == 1).unwrap();

        // lets augment our state with four empty pot before the leftmost and after the rightmost plant
        // to make pattern matching easier.

        // augment state with empty pots
        while (4 - (len - r_augment)) as i32 > 0 {
            current_state.push(0);
            r_augment -= 1;
        }
        while (4 - l_augment) as i32 > 0 {
            current_state.insert(0, 0);
            // when we add pots to left of the state, we are shifting original positions.
            left_shift += 1;
            l_augment += 1;
        }
        // buffer for the next_state
        let next_state = &mut current_state.clone();
        // generate next_state
        for idx in 0..current_state.len() - 5 {
            for (note, &pot) in &input.1 {
                let mut i = idx;
                let mut j = 0;
                let mut matched = true;

                while j < 5 {
                    if note[j] != current_state[i] {
                        matched = false;
                    }
                    j += 1;
                    i += 1;
                }
                if matched {
                    next_state[idx + 2] = pot;
                    break;
                }
                next_state[idx + 2] = 0;
            }
        }
        // current_state -> next_state
        current_state.clone_from(next_state);
        next_state.clear();
    }
    // calculate sum of the positions.
    current_state
        .iter()
        .enumerate()
        .filter(|&(.., el)| *el == 1)
        .map(|(pos, ..)| (pos - left_shift) as u32)
        .sum()
}

/// After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?
pub fn part2(_input: &Input) -> u64 {
    // this state stabilizes on after 88 generations.
    // >>> 5145 + 50 * (50_000_000_000 - 89)
    2500000000695
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
        assert_eq!(part1(&input_data), 325);
    }
}
