[![Build Status](https://travis-ci.org/rodrimati1992/repr_offset_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/repr_offset_crates)
[![](https://img.shields.io/crates/v/repr_offset.svg)][crates-io]
[![](https://docs.rs/repr_offset/badge.svg)][api-docs]


`repr_offset` allows computing and safely using field offsets from types with a stable layout.

Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.

# Features 

These are some of the features this library provides:

- Computing the offsets of all the fields in a struct with the [`unsafe_offset_constants`] macro.

- Using the [`FieldOffset`] type (how offsets are represented),
to get a pointer (or reference) to a field from a pointer to the struct.

# Examples 

For **examples** you can look at
[the examples section of the documentation for the root module of the repr_offset crate
](https://docs.rs/repr_offset/*/repr_offset/index.html#root-mod-examples)

# Future plans

Adding a derive macro that does what `unsafe_offset_constants` does.

# no-std support

This library is unconditionally `#![no_std]`,and that is unlikely to change in the future.

# Minimum Rust version

This crate support Rust back to 1.33.

# License

Licensed under the Zlib license