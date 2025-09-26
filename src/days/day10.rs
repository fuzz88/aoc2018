//! ## --- Day 10: The Stars Align ---
//!
//! Manually iterated around minimal-height of the `BoundingBox`.
//! The target height was found quickly. Don't know what else to say.

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

/// What message will eventually appear in the sky?
pub fn part1(input: &(Vec<Point>, Vec<Velocity>)) -> String {
    let mut points = input.0.clone();
    let velocities = input.1.clone();
    let mut result = String::new();

    loop {
        points.iter_mut().zip(velocities.iter()).for_each(|(p, v)| *p = *p + *v);
        let mut bounding_box = BoundingBox::from(points.as_slice());

        // actually, this height was choosen empirically
        if bounding_box.bottom - bounding_box.top == 11 {
            // got a match
            // normalizing coordinates to (0, 0) as left-top corner.
            // for printing.
            let dy = bounding_box.top - 2;

            for p in &mut points {
                p.y -= dy;
            }
            bounding_box.bottom -= dy;
            bounding_box.top -= dy - 1;

            let dx = bounding_box.left;

            for p in &mut points {
                p.x -= dx;
            }
            bounding_box.right -= dx;
            bounding_box.left -= dx - 1;

            for y in bounding_box.top..bounding_box.bottom {
                for x in bounding_box.left..bounding_box.right {
                    if points.contains(&Point { x, y }) {
                        result.push('#');
                    } else {
                        result.push('.');
                    }
                }
                result.push('\n');
            }
            break;
        }
    }

    result
}

/// Exactly how many seconds would they have needed to wait for that message to appear?
pub fn part2(input: &(Vec<Point>, Vec<Velocity>)) -> u32 {
    let mut points = input.0.clone();
    let velocities = input.1.clone();

    let mut seconds = 0;

    loop {
        seconds += 1;
        points.iter_mut().zip(velocities.iter()).for_each(|(p, v)| *p = *p + *v);
        let bounding_box = BoundingBox::from(points.as_slice());

        // actually, this height was choosen empirically
        if bounding_box.bottom - bounding_box.top == 11 {
            return seconds;
        }
    }
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
