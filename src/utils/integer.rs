//! ## Integer trait. Used to restrict types passed for parsing.

pub trait Integer {}

macro_rules! empty_trait {
    ($name:ident for $($t:ty)+) => ($(impl $name for $t {})+)
}

empty_trait!(Integer for u32 i32);
