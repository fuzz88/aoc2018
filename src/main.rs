#![feature(duration_millis_float)]
use aoc2018::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn main() {
    println!("--- Advent of Code 2018 ---");

    let day_id = match env::args().nth(1) {
        Some(arg) => arg[3..].parse::<u8>().ok(),
        None => None,
    };

    let days = days();
    let days: Vec<_> = days.iter().filter(|day| day_id.is_none_or(|id| id == day.id)).collect();

    for day in days {
        println!();
        println!("day{:02}:", day.id);
        println!("-----------------------------------------------");

        if let Ok(input_data) = fs::read_to_string(&day.input_file) {
            let (part1, part2, duration1, duration2) = (day.runner)(input_data);

            println!("{:<32} [{:>10}ms]", part1, duration1.as_millis_f32());
            println!("{:<32} [{:>10}ms]", part2, duration2.as_millis_f32());
        } else {
            eprintln!("Missing input: {}", day.input_file.display());
        }

        println!("-----------------------------------------------");
    }
}

struct Day {
    id: u8,
    input_file: PathBuf,
    runner: fn(String) -> (String, String, Duration, Duration),
}

macro_rules! run {
    ($($day:tt),*) => {
        fn days() -> Vec<Day> {
            vec![$({
                let day = stringify!($day);
                let input_file = Path::new("inputs").join(day).join("input").with_extension("txt");

                let runner = |data: String| {
                    use days::$day::*;

                    let input = parse(&data);

                    let instant = Instant::now();

                    let part1 = part1(&input);
                    let duration1 = instant.elapsed();

                    let part2 = part2(&input);
                    let duration2 = instant.elapsed();

                    (part1.to_string(), part2.to_string(), duration1, duration2 - duration1)

                };

                Day {id: day[3..].parse::<u8>().unwrap(), input_file, runner}
            },)*]
        }
    }
}

run!(day01, day02, day03);
