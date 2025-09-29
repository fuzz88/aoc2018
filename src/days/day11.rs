//! ## --- Day 11: Chronal Charge ---
//!

pub fn parse(input: &str) -> u32 {
    input.parse().unwrap()
}

pub fn part1(input: &u32) -> String {
    let x_y = String::new();

    x_y
}

pub fn part2(input: &u32) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "18";
        let input_data = parse(input);

        assert_eq!(input_data, 18);
    }
}
