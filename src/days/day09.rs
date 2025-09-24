//! ## --- Day 9: Marble Mania ---
//!
//! Using `VecDeque` gives 100x speedup (0.5ms vs 50ms) comparing to `Vec`

use std::collections::{HashMap, VecDeque};

type Scores = HashMap<u32, u32>;

#[rustfmt::skip]
pub fn parse(input: &str) -> (u32, u32) {
    // it's simplier not to use iter_unsigned here.
    let mut components = input.split_whitespace();
    (
        components.next().unwrap().parse().unwrap(),
        components.nth(5).unwrap().parse().unwrap()
    )
}

/// What is the winning Elf's score?
pub fn part1(input: &(u32, u32)) -> u32 {
    let player_count = input.0;
    let last_marble = input.1;

    let mut scores = Scores::new();
    let mut marbles = [VecDeque::new(), VecDeque::new()];

    // first marble placed, no player involved
    let mut current_marble = 0;
    marbles[0].push_back(current_marble);

    while current_marble != last_marble + 1 {
        // player places
        current_marble += 1;
        let current_player = current_marble % player_count;

        if current_marble % 23 == 0 {
            // rewind 7 marbles
            (0..7).for_each(|_| {
                if let Some(marble) = marbles[0].pop_back() {
                    marbles[1].push_front(marble);
                }
            });

            // remove and score this marble with the current one
            let score = scores.entry(current_player).or_insert(0);
            *score += current_marble + marbles[0].pop_back().unwrap();

            // set the current index properly after removing
            if let Some(marble) = marbles[1].pop_front() {
                marbles[0].push_back(marble);
            }

            // 23-ish marble not added
            continue;
        }

        // marble added after next
        if let Some(marble) = marbles[1].pop_front() {
            marbles[0].push_back(marble);
        }

        // place the current marble
        marbles[0].push_back(current_marble);

        // shift the current index in a circular way
        if marbles[1].is_empty()
            && let Some(marble) = marbles[0].pop_front()
        {
            marbles[1].push_front(marble);
        }
    }

    *scores.values().max().unwrap()
}

/// What would the new winning Elf's score be if the number of the last marble were 100 times larger?
pub fn part2(input: &(u32, u32)) -> u32 {
    part1(&(input.0, input.1 * 100))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "9 players; last marble is worth 25 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 32);
        let input = "10 players; last marble is worth 1618 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 8317);
        let input = "13 players; last marble is worth 7999 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 146373);
        let input = "17 players; last marble is worth 1104 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 2764);
        let input = "21 players; last marble is worth 6111 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 54718);
        let input = "30 players; last marble is worth 5807 points";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), 37305);
    }
}
