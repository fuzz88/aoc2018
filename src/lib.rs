//! # Advent of Code 2018, Rust, structured and *somehow* optimized.
//!
//! [![badge]][link]
//!
//! [badge]: https://img.shields.io/badge/github-blue?style=for-the-badge&logo=github&labelColor=grey
//! [link]:  https://github.com/fuzz88/aoc2018

#[doc = "Gadgets for common tasks."]
pub mod utils {
    pub mod integer;
    pub mod parse;
    pub mod point;
}

macro_rules! days {
    ($($day:ident),*) => {
        #[doc = "Solutions, actually."]
        pub mod days {
            $(pub mod $day;)*
        }
    };
}

days!(day01, day02, day03, day04, day05, day06, day07, day08, day09, day10);
