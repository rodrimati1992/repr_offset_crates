[![Build Status](https://travis-ci.org/rodrimati1992/repr_offset_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/repr_offset_crates)
[![crates-io](https://img.shields.io/crates/v/repr_offset.svg)](https://crates.io/crates/repr_offset)
[![api-docs](https://docs.rs/repr_offset/badge.svg)](https://docs.rs/repr_offset/0.0.1)



`repr_offset` allows computing and safely using field offsets from types with a stable layout.

Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.

# Features 

These are some of the features this library provides:

- Computing the offsets of all the fields in a struct with the [`unsafe_struct_field_offsets`] macro.

- Using the [`FieldOffset`] type (how offsets are represented),
to get a pointer (or reference) to a field from a pointer to the struct.

# Examples 

For **examples** you can look at
[the examples section of the documentation for the root module of the repr_offset crate
](https://docs.rs/repr_offset/*/repr_offset/index.html#root-mod-examples)

# Future plans

Adding a derive macro that does what `unsafe_struct_field_offsets` does.

# no-std support

This library is unconditionally `#![no_std]`,and that is unlikely to change in the future.

# Minimum Rust version

This crate support Rust back to 1.34.0 with only documentation tests,
and back to 1.38.0 with all the tests.

For some reason, compiling all the tests in Rust versions before 1.38.0 causes the
compiler to use an unbounded amount of memory, or overflow the stack.
(depending on the compiler version).

# License

Licensed under the Zlib license

[`unsafe_struct_field_offsets`]:
https://docs.rs/repr_offset/*/repr_offset/macro.unsafe_struct_field_offsets.html

[`FieldOffset`]: 
https://docs.rs/repr_offset/*/repr_offset/struct.FieldOffset.html





