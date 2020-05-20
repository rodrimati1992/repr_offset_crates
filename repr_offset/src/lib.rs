//! For computing and using the offset of `#[repr(C)]` structs.
//!
//! Also supports structs with the `#[repr(packed)]` and `#[repr(align(...))]` attributes.
//!
//!
//!
//! # Examples
//!
//! ### Ìnitialization
//!
//! This example demonstrates how you can:
//!
//! - Use the `unsafe_offset_constants` macro to declare associated constants with
//! the field offsets.
//!
//! - Initialize an uninitialized struct you get as a parameter.
//!
//! This example only compiles since Rust 1.36 because it uses `MaybeUninit`.
//!
#![cfg_attr(rust_1_36, doc = "```rust")]
#![cfg_attr(not(rust_1_36), doc = "```ignore")]
//!
//! use std::mem::MaybeUninit;
//!
//! use repr_offset::{unsafe_offset_constants, Aligned};
//!
//! fn main(){
//!     unsafe {
//!         let mut foo = MaybeUninit::uninit();
//!         initialize_foo(foo.as_mut_ptr());
//!         assert_eq!(
//!             foo.assume_init(),
//!             Foo{ name: "foo".to_string(), x: 13, y: 21 }
//!         );
//!     }
//! }
//!
//! /// Initializes a `Foo` through a raw pointer.
//! ///
//! /// # Safety
//! ///
//! /// Callers must pass a pointer to a uninitialized memory with the
//! /// size and alignment of `Foo`
//! unsafe fn initialize_foo(this: *mut Foo){
//!     Foo::OFFSET_NAME.get_raw_mut(this).write("foo".into());
//!     Foo::OFFSET_X.get_raw_mut(this).write(13);
//!     Foo::OFFSET_Y.get_raw_mut(this).write(21);
//! }
//!
//! #[repr(C)]
//! #[derive(Debug, PartialEq)]
//! pub struct Foo{
//!     pub name: String,
//!     pub x: u32,
//!     pub y: u32,
//! }
//!
//! // This macro is unsafe to invoke because you have to ensure that:
//! // - All fields are listed,in declaration order.
//! // - The `packing` parameter is `Packed` is the struct is `#[repr(C,packed)]`,
//! //   and `Aligned` if it's not.
//! unsafe_offset_constants!{
//!     packing = Aligned,
//!
//!     impl[] Foo {
//!         pub const OFFSET_NAME:String;
//!         pub const OFFSET_X:u32;
//!         pub const OFFSET_Y:u32;
//!     }
//! }
//!
//!
//!
//! ```

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
