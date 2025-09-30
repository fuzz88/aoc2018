//! ## --- Day 11: Chronal Charge ---
//!
//! Yeah. Solution is incomplete. We need shift the square to not recalculate sum of levels entirely. Maybe there is also the reason why the largest in power squares is relatively so small in size.
//!
//! TODO: shift square.
//!
//! We are checking squares up to size of 20 for now. Enough to find the answer by checking in into
//! Advent of Code, but not enough to reason about answer's validity.

pub fn parse(input: &str) -> u32 {
    input.trim().parse().unwrap()
}

#[inline]
fn power_level(x: usize, y: usize, serial_number: u32) -> i32 {
    (((x + 10) * y + serial_number as usize) * (x + 10) / 100 % 10 - 5) as i32
}

#[inline]
fn find_largest(field: &[i32; 90000], size: usize) -> (usize, usize, i32) {
    let mut result = (0, 0, 0);
    let mut sum_max = i32::MIN;

    for y in 1..=(300 - size + 1) {
        for x in 1..=(300 - size + 1) {
            let mut sum: i32 = 0;
            for dy in y..y + size {
                for dx in x..x + size {
                    sum += field[(dy - 1) * 300 + (dx - 1)];
                }
            }
            if sum > sum_max {
                sum_max = sum;
                result = (x, y, sum_max);
            }
        }
    }

    result
}
/// What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?
pub fn part1(input: &u32) -> String {
    let mut field = [0; 300 * 300];
    let serial_number = *input;

    for y in 1..=300 {
        for x in 1..=300 {
            field[(y - 1) * 300 + (x - 1)] = power_level(x, y, serial_number);
        }
    }

    let largest = find_largest(&field, 3);

    format!("{},{}", largest.0, largest.1)
}

/// What is the X,Y,size identifier of the square with the largest total power?
pub fn part2(input: &u32) -> String {
    let mut x_y = String::new();
    let mut field = [0; 300 * 300];
    let serial_number = *input;

    for y in 1..=300 {
        for x in 1..=300 {
            field[(y - 1) * 300 + (x - 1)] = power_level(x, y, serial_number);
        }
    }

    let mut sum_max_total = i32::MIN;

    for size in 1..=20 {
        let sum = find_largest(&field, size);
        if sum_max_total < sum.2 {
            sum_max_total = sum.2;
            x_y = format!("{},{},{}", sum.0, sum.1, size);
        }
    }

    x_y
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
        assert_eq!(part2(&input_data), "90,269,16");
    }
}
