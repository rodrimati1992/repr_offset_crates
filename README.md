[![Rust](https://github.com/rodrimati1992/repr_offset_crates/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/repr_offset_crates/actions)
[![crates-io](https://img.shields.io/crates/v/repr_offset.svg)](https://crates.io/crates/repr_offset)
[![api-docs](https://docs.rs/repr_offset/badge.svg)](https://docs.rs/repr_offset/*)



`repr_offset` allows computing and safely using field offsets from types with a stable layout.

Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.

# Features

These are some of the features this library provides:

- The [`ReprOffset`] derive macro, which outputs associated constants with the
offsets of fields.<br>

- Using the [`FieldOffset`] type (how offsets are represented),
with methods for operating on a field through a pointer to the struct,
including getting a reference(or pointer) to the field.

- Use the [`unsafe_struct_field_offsets`] macro as an alternative to the
[`ReprOffset`] derive macro, most useful when the "derive" feature is disabled.


# Examples 

For **examples** you can look at
[the examples section of the documentation for the root module of the repr_offset crate
](https://docs.rs/repr_offset/*/repr_offset/index.html#root-mod-examples)

# Future plans

None for now.

# Cargo features

These are the cargo features in repr_offset:

- `derive` (enabled by default): 
Re-exports the `ReprOffset` derive macro from the `repr_offset_derive` crate.


You can disable default features by using `default_features = false`,
eg:`repr_offset = { version = "0.1", default_features = false }`.

# no-std support

This library is unconditionally `#![no_std]`,and that is unlikely to change in the future.

# Minimum Rust version

This crate support Rust back to 1.41.0.

# License

Licensed under the Zlib license

[`ReprOffset`]:
https://docs.rs/repr_offset/*/repr_offset/docs/repr_offset_macro/index.html

[`unsafe_struct_field_offsets`]:
https://docs.rs/repr_offset/*/repr_offset/macro.unsafe_struct_field_offsets.html

[`FieldOffset`]: 
https://docs.rs/repr_offset/*/repr_offset/struct.FieldOffset.html





