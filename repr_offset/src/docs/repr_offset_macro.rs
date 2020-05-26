//! The `ReprOffset` derive macro generates associated constants with the offset of every field.
//!
//! The `ReprOffset` derive is from the `repr_offset_derive` crate,
//! and is re-exported by this crate when
//! the "derive" feature is enabled (it's enabled by default).
//!
//! # Generated offsets
//!
//! The generated offset associated constants (by default):
//!
//! - Their names have the  `OFFSET_` prefix, with capitalized field names as a suffix.
//! <br>
//! Eg: `OFFSET_FOO`, `OFFSET_VALID_UNTIL.`
//!
//! - Are [`FieldOffset`]s, which has safe methods for operating on the field at that offset.
//!
//! - Have the same privacy as the field.
//!
//! # Valid Representation Attributes
//!
//! These are the valid representation attributes:
//!
//! - `#[repr(C)]`
//!
//! - `#[repr(transparent)]`: This is treated the same as `#[repr(C)]`.
//!
//! - `#[repr(C, align(1000))]`
//!
//! - `#[repr(C, packed)]`
//!
//! - `#[repr(C, packed(1000))]`
//!
//! One of those must be used,otherwise the derive macro will error.
//!
//!
//! # Container Attributes
//!
//! ### `#[roff(usize_offsets)]`
//!
//! Changes the generated offset associated constants from [`FieldOffset`] to `usize`.
//!
//! Example:
//! ```rust
#![cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#![cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
//!
//! #[repr(C)]
//! #[derive(ReprOffset)]
//! #[roff(usize_offsets)]
//! struct Foo{
//!     x: u8,
//!     y: u64,
//!     z: String,
//! }
//!
//! let _: usize = Foo::OFFSET_X;
//! let _: usize = Foo::OFFSET_Y;
//! let _: usize = Foo::OFFSET_Z;
//!
//! ```
//!
//! ### `#[roff(bound = "T: Foo")]`
//!
//! This attribute adds a constraint to the generated impl block that defines
//! the field offset constants.
//!
//! Examples:
//!
//! - `#[roff(bound = "T: 'a")]`
//!
//! - `#[roff(bound = "U: Foo")]`
//!
//! # Field attributes
//!
//! ### `#[roff(offset = "fooo")]`
//!
//! Changes the name of the generated offset for the field.
//!
//! Example:
//! ```rust
#![cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#![cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
//! use repr_offset::{Aligned, FieldOffset};
//!
//! #[repr(C)]
//! #[derive(ReprOffset)]
//! struct Foo{
//!     x: u8,
//!     y: u64,
//!     #[roff(offset = "this_is_lowercase")]
//!     z: String,
//! }
//!
//! let _: FieldOffset<Foo, u8, Aligned> = Foo::OFFSET_X;
//! let _: FieldOffset<Foo, u64, Aligned> = Foo::OFFSET_Y;
//! let _: FieldOffset<Foo, String, Aligned> = Foo::this_is_lowercase;
//!
//! ```
//!
//! # Container or Field attributes
//!
//! ### `#[roff(offset_prefix = "FOO" )]`
//!
//! Changes the prefix of the name of the generated offset(s) for the field(s).
//!
//! When used on the type, it uses this as the default prefix of all
//! the offset constants for the fields.
//!
//! When used on a field,
//! it overrides the prefix of the name of the offset constant for the field.
//!
//! Example:
//! ```rust
#![cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#![cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
//! use repr_offset::{FieldOffset, Unaligned};
//!
//! #[repr(C, packed)]
//! #[derive(ReprOffset)]
//! #[roff(offset_prefix = "OFF_")]
//! struct Foo{
//!     x: u8,
//!     y: u64,
//!     // This overrides the `offset_prefix` attribute above.
//!     #[roff(offset_prefix = "OOO_")]
//!     z: String,
//! }
//!
//! let _: FieldOffset<Foo, u8, Unaligned> = Foo::OFF_X;
//! let _: FieldOffset<Foo, u64, Unaligned> = Foo::OFF_Y;
//! let _: FieldOffset<Foo, String, Unaligned> = Foo::OOO_Z;
//!
//! ```
//!
//!
//! [`FieldOffset`]: ../../struct.FieldOffset.html
//!
