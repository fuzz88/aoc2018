//! ## --- Day 9: Marble Mania ---
//!

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
    0
}

pub fn part2(input: &(u32, u32)) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "9 players; last marble is worth 25 points";
        let input_data = parse(input);
        assert_eq!(input_data, (9, 25));
        assert_eq!(part1(&input_data), 32);
    }
}
