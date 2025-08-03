//! ## --- Day 4: Repose Record ---
//!
//! Apparently there are guards which don't sleep.

use crate::utils::parse::*;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum GuardEvent {
    BeginsShift,
    FallsAsleep,
    WakesUp,
}

/// (Month, day, hour, minute): let's assume that all events are of the same year.
#[derive(Eq, Ord, Clone, Copy, PartialOrd, PartialEq)]
struct Timestamp(u8, u8, u8, u8);

/// Represents line of input.
pub struct Record(Timestamp, Option<u32>, GuardEvent);

/// Each guard will have statistics for minutes he slept.
///
/// guard => (minute => times)
type Timetable = HashMap<u32, HashMap<u8, u32>>;

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

        let event = info.split_whitespace().last().unwrap();
        let event = match event {
            "shift" => GuardEvent::BeginsShift,
            "asleep" => GuardEvent::FallsAsleep,
            "up" => GuardEvent::WakesUp,
            _ => unreachable!(),
        };

        let mut info_iter = info.iter_unsigned();
        let guard_id = info_iter.next();

        Record(Timestamp(month, day, hour, minute), guard_id, event)
    }
}

pub fn parse(input: &str) -> Vec<Record> {
    let as_record = |line: &str| line.split("] ").collect();

    let mut records: Vec<Record> = input.lines().map(as_record).collect();

    records.sort_by_key(|item| item.0);

    records
}

fn calculate_timetable(input: &[Record]) -> Timetable {
    let mut timetable = Timetable::new();

    let first_guard = input[0].1.unwrap();
    let mut guard_stats = timetable.entry(first_guard).or_default();

    for idx in 1..input.len() {
        // if met new guard, start collecting it's statistics.
        let record_guard = input[idx].1;
        if let Some(next_guard) = record_guard {
            guard_stats = timetable.entry(next_guard).or_default();
        }

        // FallAsleep is always previous to WakesUp
        let record_event = &input[idx].2;
        if *record_event == GuardEvent::WakesUp {
            // if hour is not 0, then minute is 0,
            // for example, 23:58 counts as 00:00.
            let fallasleep_minute = if input[idx - 1].0.2 != 0 { 0 } else { input[idx - 1].0.3 };
            let wakesup_minute = if input[idx].0.2 != 0 { 0 } else { input[idx].0.3 };

            for minute in fallasleep_minute..wakesup_minute {
                *guard_stats.entry(minute).or_insert(0) += 1;
            }
        }
    }

    timetable
}

/// Strategy 1: Find the guard that has the most minutes asleep. What minute does that guard spend asleep the most?
pub fn part1(input: &[Record]) -> u32 {
    let timetable = calculate_timetable(input);

    let mut sleepy_guard = 0;
    let mut that_minute = 0;
    let mut max_slept = 0;

    for (guard, minutes) in timetable {
        let total_sleep = minutes.values().sum();

        if max_slept < total_sleep {
            max_slept = total_sleep;
            sleepy_guard = guard;
            that_minute = *minutes.iter().max_by_key(|el| el.1).unwrap().0 as u32;
        }
    }

    sleepy_guard * that_minute
}

/// Strategy 2: Of all guards, which guard is most frequently asleep on the same minute?
pub fn part2(input: &[Record]) -> u32 {
    let timetable = calculate_timetable(input);

    let mut sleepy_guard = 0;
    let mut that_minute = 0;
    let mut most_times = 0;

    for (guard, minutes) in timetable {
        if let Some((&minute, &freq)) = minutes.iter().max_by_key(|el| el.1)
            && most_times < freq
        {
            most_times = freq;
            sleepy_guard = guard;
            that_minute = minute as u32;
        }
    }

    sleepy_guard * that_minute
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
        assert_eq!(part1(&input_data), 240);
        assert_eq!(part2(&input_data), 4455);
    }
}
