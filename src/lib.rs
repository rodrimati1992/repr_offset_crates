#![no_std]
#![cfg_attr(feature = "priv_raw_ref", feature(raw_ref_op))]

#[doc(hidden)]
pub mod field_offset;

#[macro_use]
mod macros;

#[cfg(feature = "testing")]
#[macro_use]
mod test_macros;

mod utils;

#[cfg(feature = "testing")]
#[macro_use]
pub mod tests;

#[cfg(feature = "testing")]
pub mod types_for_tests;

pub use self::field_offset::{Aligned, FieldOffset, Packed};

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }
