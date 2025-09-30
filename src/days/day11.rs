//! ## --- Day 11: Chronal Charge ---
//!

pub fn parse(input: &str) -> u32 {
    input.trim().parse().unwrap()
}

#[inline]
fn power_level(x: usize, y: usize, serial_number: u32) -> i32 {
    (((x + 10) * y + serial_number as usize) * (x + 10) / 100 % 10 - 5) as i32
}

pub fn part1(input: &u32) -> String {
    let mut x_y = String::new();
    let mut field = [0; 300 * 300];
    let serial_number = *input;

    for y in 1..=300 {
        for x in 1..=300 {
            field[(y - 1) * 300 + (x - 1)] = power_level(x, y, serial_number);
        }
    }

    let mut sum_max = i64::MIN;

    for y in 1..=298 {
        for x in 1..=298 {
            let mut sum: i64 = 0;
            for dy in y..=y + 2 {
                for dx in x..=x + 2 {
                    sum += field[(dy - 1) as usize * 300 + (dx - 1) as usize] as i64;
                }
            }
            if sum > sum_max {
                sum_max = sum;
                x_y = format!("{},{}", x, y);
            }
        }
    }

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
        let input = "18\n";
        let input_data = parse(input);
        assert_eq!(input_data, 18);
        assert_eq!(part1(&input_data), "33,45");
    }
}
