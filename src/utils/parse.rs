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

trait IsDigit {
    fn is_digit(&self) -> bool;
}

impl IsDigit for u8 {
    fn is_digit(&self) -> bool {
        self.wrapping_sub(b'0') < 10
    }
}

fn parse_integer<const IS_SIGNED: bool, T: FromStr + Integer>(
    data: &str,
    cursor: &mut usize,
) -> Option<T> {
    let bytes = data.as_bytes();
    if *cursor == bytes.len() {
        return None;
    }

    let begin = loop {
        if bytes[*cursor].is_digit() {
            if IS_SIGNED && *cursor > 0 && bytes[*cursor - 1] == b'-' {
                break *cursor - 1;
            }
            break *cursor;
        }
        *cursor += 1;

        if *cursor == bytes.len() {
            return None;
        }
    };
    let end = loop {
        if *cursor == bytes.len() - 1 {
            break *cursor + 1;
        }
        if bytes[*cursor].is_digit() {
            *cursor += 1;
        } else {
            break *cursor;
        }
    };
    *cursor += 1;

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
