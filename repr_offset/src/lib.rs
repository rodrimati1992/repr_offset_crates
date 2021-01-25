//! `repr_offset` allows computing and safely using field offsets from types with a stable layout.
//!
//! Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.
//!
//! # Features
//!
//! These are some of the features this library provides:
//!
//! - The [`ReprOffset`] derive macro, which outputs associated constants with the
//! offsets of fields.<br>
//!
//! - Using the [`FieldOffset`] type (how offsets are represented),
//! with methods for operating on a field through a pointer to the struct,
//! including getting a reference(or pointer) to the field.
//!
//! - Use the [`unsafe_struct_field_offsets`] macro as an alternative to the
//! [`ReprOffset`] derive macro, most useful when the "derive" feature is disabled.
//!
//! # Dependencies
//!
//! This library re-exports the [`ReprOffset`] derive macro from the
//! `repr_offset_derive` crate when the "derive" feature is enabled
//! (it's enabled is the default).
//!
//! If you don't need the derive macro,
//! you can disable the default feature in the Cargo.toml file with
//! `repr_offset = { version = "....", default_features = false }`,
//! making a clean compile of this crate take one to three seconds(depends on the machine).
//!
//! <span id="root-mod-examples"></span>
//! # Examples
//!
//! ### Derivation
//!
//! This example demonstrates:
//!
//! - Deriving the field offset constants with the [`ReprOffset`] derive macro.
//!
//! - Moving out *unaligned* fields through a raw pointer.
//!
//! ```rust
#![cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#![cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
//!
//! use std::mem::ManuallyDrop;
//!
//! #[repr(C, packed)]
//! #[derive(ReprOffset)]
//! struct Packed{
//!     x: u8,
//!     y: u64,
//!     z: String,
//! }
//!
//! let mut this = ManuallyDrop::new(Packed{
//!     x: 5,
//!     y: 8,
//!     z: "oh,hi".to_string(),
//! });
//!
//! let ptr: *mut Packed = &mut *this;
//!
//! unsafe{
//!     assert_eq!( Packed::OFFSET_X.read(ptr), 5 );
//!     assert_eq!( Packed::OFFSET_Y.read(ptr), 8 );
//!     assert_eq!( Packed::OFFSET_Z.read(ptr), "oh,hi".to_string() );
//! }
//!
//! ```
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
//! ```rust
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
//!     Foo::OFFSET_NAME.raw_get_mut(this).write("foo".into());
//!     Foo::OFFSET_X.raw_get_mut(this).write(13);
//!     Foo::OFFSET_Y.raw_get_mut(this).write(21);
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
//! // - The `alignment` parameter is `Unaligned` if the struct is `#[repr(C,packed)]`,
//! //   and `Aligned` if it's not.
//! unsafe_struct_field_offsets!{
//!     alignment =  Aligned,
//!
//!     impl[] Foo {
//!         pub const OFFSET_NAME, name: String;
//!         pub const OFFSET_X, x: u32;
//!         pub const OFFSET_Y, y: u32;
//!     }
//! }
//!
//!
//!
//! ```
//!
//!
//! [`ReprOffset`]: ./docs/repr_offset_macro/index.html
//! [`unsafe_struct_field_offsets`]: ./macro.unsafe_struct_field_offsets.html
//! [`FieldOffset`]: ./struct.FieldOffset.html
//!
#![no_std]
#![cfg_attr(feature = "priv_raw_ref", feature(raw_ref_op))]
// TODO: uncomment
// #![deny(clippy::missing_safety_doc)]
// #![deny(clippy::shadow_unrelated)]
// #![deny(clippy::wildcard_imports)]
// #![deny(missing_docs)]

#[doc(hidden)]
pub extern crate self as repr_offset;

#[macro_use]
mod internal_macros;

#[macro_use]
mod macros;

#[cfg(feature = "testing")]
#[macro_use]
mod test_macros;

pub mod docs;

pub mod offset_calc;

pub mod alignment;

pub mod privacy;

/// Types used for examples,
///
/// These are in the docs purely so that documentation examples only use
/// types that are documented.
///
/// You can only use items from this module when the "for_examples" feature is enabled.
pub mod for_examples {
    #[doc(inline)]
    #[cfg(any(feature = "for_examples", doc))]
    pub use crate::for_examples_inner::*;
}

#[doc(hidden)]
#[cfg(any(feature = "for_examples", doc))]
pub mod for_examples_inner;

mod struct_field_offset;

pub mod ext;

pub mod get_field_offset;

pub mod utils;

#[cfg(feature = "testing")]
pub mod types_for_tests;

pub use tstr;

/// This derive macro [is documented in here](./docs/repr_offset_macro/index.html)
#[doc(inline)]
#[cfg(feature = "derive")]
pub use repr_offset_derive::ReprOffset;

pub use self::{
    alignment::{Aligned, Unaligned},
    ext::{ROExtAcc, ROExtOps, ROExtRawAcc, ROExtRawMutAcc, ROExtRawMutOps, ROExtRawOps},
    get_field_offset::{FieldType, GetPubFieldOffset},
    struct_field_offset::FieldOffset,
};

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }

// DO NOT USE THIS OUTSIDE MACROS OF THIS CRATE
#[doc(hidden)]
pub mod pmr {
    pub use core::marker::PhantomData;

    pub use crate::struct_field_offset::FOAssertStruct;

    pub use crate::get_field_offset::{
        loop_create_fo, loop_create_mutref, loop_create_val, FieldOffsetWithVis, GetFieldOffset,
        GetPubFieldOffset, ImplsGetFieldOffset, InitPrivOffset,
    };
}
