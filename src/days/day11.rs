//! ## --- Day 11: Chronal Charge ---
//!
//! Got a hint to use https://en.wikipedia.org/wiki/Summed-area_table
//!

// Holds largest power square for the size.
pub struct MaxSquare {
    x: usize,
    y: usize,
    size: usize,
    power: i32,
}

#[inline]
fn power_level(x: usize, y: usize, serial_number: usize) -> i32 {
    (((x + 10) * y + serial_number) * (x + 10) / 100 % 10 - 5) as i32
}

pub fn parse(input: &str) -> Vec<MaxSquare> {
    let serial_number = input.trim().parse().unwrap();

    let mut sat = vec![0; 301 * 301];

    for y in 1..301 {
        for x in 1..301 {
            let index = 301 * y + x;
            // I(x, y) = i(x, y) + I(x, y - 1) + I(x - 1, y) - I(x - 1, y - 1)
            sat[index] = power_level(x, y, serial_number) + sat[index - 1] + sat[index - 301]
                - sat[index - 302];
        }
    }
}

/// What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?
pub fn part1(input: &[MaxSquare]) -> String {
    let MaxSquare { x, y, .. } = input.iter().find(|s| s.size == 3).unwrap();
    format!("{x},{y}")
}

/// What is the X,Y,size identifier of the square with the largest total power?
pub fn part2(input: &[MaxSquare]) -> String {
    let MaxSquare { x, y, size, .. } = input.iter().max_by_key(|s| s.power).unwrap();
    format!("{x},{y},{size}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "18\n";
        let input_data = parse(input);
        assert_eq!(part1(&input_data), "33,45");
        assert_eq!(part2(&input_data), "90,269,16");
    }
}
