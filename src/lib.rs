#![no_std]

#[doc(hidden)]
pub mod field_offset;

#[macro_use]
mod macros;

#[macro_use]
mod test_macros;

mod utils;

pub use self::field_offset::{Aligned, FieldOffset, Packed};
