use std::env;
// use std::fs;
// use std::time;
use aoc2018::*;
use std::path::Path;

fn main() {
    println!("--- Advent of Code 2018 ---");

    if let Some(day) = env::args().nth(1) {
        let input_file = Path::new("inputs").join(day).join("input").with_extension("txt");
        println!("{}", input_file.display());
        println!("run all year");
    }
}
