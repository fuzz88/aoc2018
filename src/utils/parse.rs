//! Extracts signed integers from a string, passing by all the surrounding or delimiting stuff.
//!
//! Text representation of input data in puzzles is always ASCII.
//!
//! It is common to have input data represented as sequence of integers, separated by newlines,
//! whitespaces or surrounded by descriptive text. For example:
//!
//! ```none
//! 12 Elves have 24 apples at -5 celsius.
//! ```
//!
//! The good starting point is to have facilities to iterate over these integers.
