//! ## --- Day 4: Repose Record ---
//!
//!

use crate::utils::parse::*;

#[derive(Debug)]
pub enum GuardAction {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

#[derive(Debug)]
pub struct Record(u8, u8, u8, Option<u32>, GuardAction);

impl<'a> FromIterator<&'a str> for Record {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        let mut iter = iter.into_iter();

        let date = iter.next().unwrap();
        let mut date_iter = date.iter_unsigned();

        let _year = date_iter.next();
        let month = date_iter.next().unwrap();
        let day = date_iter.next().unwrap();
        let hour = date_iter.next().unwrap();
        let minute = date_iter.next().unwrap();

        let info = iter.next().unwrap();

        let action = info.split_whitespace().last().unwrap();
        let action = match action {
            "shift" => GuardAction::BeginsShift,
            "asleep" => GuardAction::FallsAsleep,
            "up" => GuardAction::WakesUp,
            act => unimplemented!("unknown action: {}", act),
        };

        let mut info_iter = info.iter_unsigned();
        let guard_id = info_iter.next();

        Record(month, day, if hour == 0 {minute} else {0}, guard_id, action)
    }
}

pub fn parse(input: &str) -> Vec<Record> {
    let as_record = |line: &str| line.split("] ").collect();
    input.lines().map(as_record).collect()
}

pub fn part1(input: &[Record]) -> u32 {
    0
}

pub fn part2(input: &[Record]) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let input_data = parse(input);

        assert_eq!(input_data.len(), 17);
    }
}
