//! `repr_offset` allows computing and safely using field offsets from types with a stable layout.
//!
//! Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.
//!
//! # Features
//!
//! These are some of the features this library provides:
//!
//! - Computing the offsets of all the fields in a struct with the
//! [`unsafe_struct_field_offsets`] macro.
//!
//! - Using the [`FieldOffset`] type (how offsets are represented),
//! to get a pointer (or reference) to a field from a pointer to the struct.
//!
//! <span id="root-mod-examples"></span>
//! # Examples
//!
//! ### Initialization
//!
//! This example demonstrates how you can:
//!
//! - Use the [`unsafe_struct_field_offsets`] macro to declare associated constants with
//! the field offsets.
//!
//! - Initialize an uninitialized struct by passing a pointer to it.
//!
//! This example only compiles since Rust 1.36 because it uses `MaybeUninit`.
//!
#![cfg_attr(rust_1_36, doc = "```rust")]
#![cfg_attr(not(rust_1_36), doc = "```ignore")]
//!
//! use std::mem::MaybeUninit;
//!
//! use repr_offset::{unsafe_struct_field_offsets, Aligned};
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
//! /// Callers must pass a pointer to uninitialized memory with the
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
//! // - All field types are listed,in declaration order.
//! // - The `packing` parameter is `Packed` if the struct is `#[repr(C,packed)]`,
//! //   and `Aligned` if it's not.
//! unsafe_struct_field_offsets!{
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
//!
//!
//! [`unsafe_struct_field_offsets`]: ./macro.unsafe_struct_field_offsets.html
//! [`FieldOffset`]: ./struct.FieldOffset.html
//!
#![no_std]
#![cfg_attr(feature = "priv_raw_ref", feature(raw_ref_op))]

mod field_offset;

pub mod offset_calc;

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
