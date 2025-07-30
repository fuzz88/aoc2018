//! Extracts integers from the ascii string.
//!
//! Not the fastest solution, but convenient and mine. :-]

use std::marker::PhantomData;
use std::str::FromStr;

pub trait Integer {}
impl Integer for i32 {}

pub struct ParsingSigned<'a, T> {
    data: &'a str,
    cursor: usize,
    phantom: PhantomData<T>,
}

impl<T: FromStr + Integer> Iterator for ParsingSigned<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.data.as_bytes();
        let cursor = &mut self.cursor;

        if *cursor == bytes.len() {
            return None;
        }

        let begin = loop {
            let byte = bytes[*cursor];

            if byte.wrapping_sub(b'0') < 10 {
                if *cursor > 0 && bytes[*cursor - 1] == b'-' {
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
            if bytes[*cursor].wrapping_sub(b'0') < 10 {
                *cursor += 1;
            } else {
                break *cursor;
            }
        };

        *cursor += 1;

        self.data[begin..end].parse().ok()
    }
}

pub trait ParseOps {
    fn iter_signed<T: FromStr + Integer>(&self) -> ParsingSigned<'_, T>;
}

impl ParseOps for &str {
    fn iter_signed<T: FromStr + Integer>(&self) -> ParsingSigned<'_, T> {
        ParsingSigned { data: self, cursor: 0, phantom: PhantomData }
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
}
