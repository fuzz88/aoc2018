//! Extracts integers from the ascii string.
//!
//! Not the fastest solution, but convenient and mine. :-]

use crate::utils::integer::*;
use std::marker::PhantomData;
use std::str::FromStr;

pub struct ParsingSigned<'a, T> {
    data: &'a str,
    cursor: usize,
    phantom: PhantomData<T>,
}

pub struct ParsingUnsigned<'a, T> {
    data: &'a str,
    cursor: usize,
    phantom: PhantomData<T>,
}

fn parse_integer<const IS_SIGNED: bool, T: FromStr + Integer>(
    data: &str,
    cursor: &mut usize,
) -> Option<T> {
    let bytes = data.as_bytes();

    // checks if the cursor is out-of-the data.
    if *cursor == bytes.len() {
        return None;
    }

    let (begin, end) = (
        loop {
            // searching for the first digit of the integer.
            if bytes[*cursor].is_ascii_digit() {
                // checking if the integer is signed.
                if IS_SIGNED && *cursor > 0 && bytes[*cursor - 1] == b'-' {
                    // it is signed: breaking on the sign.
                    break *cursor - 1;
                }
                // if not: breaking on the digit.
                break *cursor;
            }
            // not a digit yet. advance.
            *cursor += 1;

            // no single digit in the data.
            if *cursor == bytes.len() {
                return None;
            }
        },
        loop {
            // check if the current byte is the digit.
            if bytes[*cursor].is_ascii_digit() {
                // if the cursor is on the end of the data.
                if *cursor + 1 == bytes.len() {
                    // advance the cursor out-of-the data.
                    // the end of the data is the end of the integer.
                    *cursor += 1;
                    break *cursor;
                }
                // advance the cursor to the next byte and check.
                *cursor += 1;
            } else {
                // not a digit is the end of the integer.
                break *cursor;
            }
        },
    );
    // parsing integer if found.
    data[begin..end].parse().ok()
}

impl<T: FromStr + Integer> Iterator for ParsingSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        parse_integer::<true, T>(self.data, &mut self.cursor)
    }
}

impl<T: FromStr + Integer> Iterator for ParsingUnsigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        parse_integer::<false, T>(self.data, &mut self.cursor)
    }
}

pub trait ParseOps {
    fn iter_signed<T: FromStr + Integer>(&self) -> ParsingSigned<'_, T>;
    fn iter_unsigned<T: FromStr + Integer>(&self) -> ParsingUnsigned<'_, T>;
}

impl ParseOps for &str {
    fn iter_signed<T: FromStr + Integer>(&self) -> ParsingSigned<'_, T> {
        ParsingSigned { data: self, cursor: 0, phantom: PhantomData }
    }
    fn iter_unsigned<T: FromStr + Integer>(&self) -> ParsingUnsigned<'_, T> {
        ParsingUnsigned { data: self, cursor: 0, phantom: PhantomData }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_signed_numbers() {
        let s = String::from("123 blabla :-1-4 12-13-5 -1234565 1234 19");
        let numbers: Vec<i32> = s.as_str().iter_signed().collect();

        assert_eq!(numbers.len(), 9);
        assert_eq!(numbers, vec![123, -1, -4, 12, -13, -5, -1234565, 1234, 19]);
    }

    #[test]
    fn parse_unsigned_numbers() {
        let s = String::from("123 blabla :-1-4 12-13-5 -1234565 1234 19");
        let numbers: Vec<i32> = s.as_str().iter_unsigned().collect();

        assert_eq!(numbers.len(), 9);
        assert_eq!(numbers, vec![123, 1, 4, 12, 13, 5, 1234565, 1234, 19]);
    }
}
