//! ## --- Day 10: The Stars Align ---
//!

use crate::utils::point::*;

type Velocity = Point;

pub fn parse(input: &str) -> (Vec<Point>, Vec<Velocity>) {
    input
        .lines()
        .flat_map(|line| line.split("> ").map(Point::from))
        .array_chunks()
        .map(|[a, b]| (a, b))
        .unzip()
}

pub fn part1(input: &(Vec<Point>, Vec<Velocity>)) -> u32 {
    0
}

pub fn part2(input: &(Vec<Point>, Vec<Velocity>)) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
        assert_eq!(input.lines().count(), 31);
        let input_data = parse(input);
        assert_eq!(input_data.0.len(), 31);
        assert_eq!(input_data.1.len(), 31);
    }
}
