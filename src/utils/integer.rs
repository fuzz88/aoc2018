//! ## Integer trait. Used to restrict types passed for parsing.

pub trait Integer {}

macro_rules! integer {
    ($($t:ty),+) => ($(impl Integer for $t {})+)
}

integer!(u32, i32);
