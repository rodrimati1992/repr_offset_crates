//! `repr_offset` allows computing and safely using field offsets from types
//! with a defined layout.
//!
//! Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.
//!
//! # Features
//!
//! These are some of the features this library provides:
//!
//! - The [`ReprOffset`] derive macro, which outputs associated constants with the
//! offsets of fields, and implements the [`GetFieldOffset`] trait for each field.<br>
//!
//! - The [`FieldOffset`] type (how offsets are represented),
//! with methods for operating on a field through a pointer to the struct,
//! including getting a reference(or pointer) to the field.
//!
//! - The [`unsafe_struct_field_offsets`] macro as an alternative to the
//! [`ReprOffset`] derive macro, most useful when the "derive" feature is disabled.
//!
//! - The [`GetFieldOffset`] trait, for getting the [`FieldOffset`] for a field,
//! and the [`OFF!`], [`off`], [`PUB_OFF!`], and [`pub_off`] macros for
//! getting the [`FieldOffset`] for a field with a convenient syntax.
//!
//! - The extension traits from the [`ext`] module,
//! which define methods for operating on a field, given a [`FieldOffset`].
//!
//! <span id="root-mod-examples"></span>
//! # Examples
//!
//! ### Derivation
//!
//! This example demonstrates:
//!
//! - Deriving the field offset constants and [`GetFieldOffset`] trait
//! with the [`ReprOffset`] derive macro.
//!
//! - Moving out *unaligned* fields through a raw pointer.
//!
//! - The [`off`] macro, and an extension trait from the [`ext`] module.
//!
//! ```rust
#![cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#![cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
//!
//! use repr_offset::{ROExtRawOps, off};
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
//!
//!     // Another way to do the same, using extension traits, and macros.
//!     assert_eq!( ptr.f_read(off!(x)), 5 );
//!     assert_eq!( ptr.f_read(off!(y)), 8 );
//! }
//!
//! ```
//!
//! ### Initialization
//!
//! This example demonstrates how you can:
//!
//! - Use the [`unsafe_struct_field_offsets`] macro to declare associated constants with
//! the field offsets, and implement the [`GetFieldOffset`] trait.
//!
//! - The [`off`] macro, and an extension trait from the [`ext`] module.
//!
//! - Initialize an uninitialized struct with a functino that takes a raw pointer.
//!
//! ```rust
//!
//! use std::mem::MaybeUninit;
//!
//! use repr_offset::{
//!     unsafe_struct_field_offsets,
//!     off,
//!     Aligned, ROExtRawMutOps,
//! };
//!
//! fn main(){
//!     unsafe {
//!         let mut foo = MaybeUninit::<Foo>::uninit();
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
//!     // How it's done with the inherent associated constants declared in
//!     // the `unsafe_struct_field_offsets` macro
//!     //
//!     Foo::OFFSET_NAME.write(this, "foo".into());
//!     Foo::OFFSET_X.write(this, 13);
//!     Foo::OFFSET_Y.write(this, 21);
//!
//!     // How it's done with the extension traits from the ext module,
//!     // the `off` macro, and the `GetFieldOffset` trait:
//!     //
//!     //     this.f_write(off!(name), "foo".into());
//!     //     this.f_write(off!(x), 13);
//!     //     this.f_write(off!(y), 21);
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
//! # Dependencies
//!
//! This library re-exports the [`ReprOffset`] derive macro from the
//! `repr_offset_derive` crate when the "derive" feature is enabled,
//! this is disabled by default.
//!
//! It also reexports the `tstr` crate unconditionally, to use its `TS` macro
//! as the type parameter of the [`GetFieldOffset`] trait.
//!
//! # Cargo features
//!
//! These are the cargo features in `repr_offset`:
//!
//! - `derive` (disabled by default):
//! Enables the [`ReprOffset`] derive macro.
//! This requires the same Rust versions as `syn`, which is currently `>= 1.56.0`.
//!
//! - `"for_examples"` (disabled by default):
//! Enables the `for_examples` module, with types used in documentation examples.
//!
//! Example of using the "derive" feature::
//! ```toml
//! repr_offset = { version = "0.2", features = ["derive"] }
//! ```
//!
//! # no-std support
//!
//! This library is unconditionally `#![no_std]`, and that is unlikely to change in the future.
//!
//! # Minimum Rust version
//!
//! This crate support Rust back to 1.41.0.
//!
//!
//!
//! [`OFF!`]: ./macro.OFF.html
//! [`off`]: ./macro.off.html
//! [`PUB_OFF!`]: ./macro.PUB_OFF.html
//! [`pub_off`]: ./macro.pub_off.html
//!
//! [`ReprOffset`]: ./derive.ReprOffset.html
//! [`GetFieldOffset`]: ./get_field_offset/trait.GetFieldOffset.html
//! [`unsafe_struct_field_offsets`]: ./macro.unsafe_struct_field_offsets.html
//! [`FieldOffset`]: ./struct.FieldOffset.html
//! [`ext`]: ./ext/index.html
//!
#![no_std]
#![cfg_attr(feature = "priv_raw_ref", feature(raw_ref_op))]
#![cfg_attr(feature = "docsrs", feature(doc_cfg))]
#![allow(clippy::empty_loop)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]
#![deny(missing_docs)]

#[doc(hidden)]
pub extern crate self as repr_offset;

#[macro_use]
mod internal_macros;

#[macro_use]
mod macros;

#[cfg(feature = "testing")]
#[macro_use]
mod test_macros;

pub mod offset_calc;

pub mod alignment;

pub mod privacy;

/// Types used for examples,
///
/// These are in the docs purely so that documentation examples only use
/// types that are documented.
///
/// You can only use items from this module when the "for_examples" feature is enabled.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "for_examples")))]
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

include! {"repr_offset_macro.rs"}

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
        GetPubFieldOffset, ImplsGetFieldOffset,
    };
}
