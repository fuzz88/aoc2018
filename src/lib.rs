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
}

#[doc = "Solutions, actually."]
pub mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}
