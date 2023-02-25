[![Rust](https://github.com/rodrimati1992/repr_offset_crates/workflows/Rust/badge.svg)](https://github.com/rodrimati1992/repr_offset_crates/actions)
[![crates-io](https://img.shields.io/crates/v/repr_offset.svg)](https://crates.io/crates/repr_offset)
[![api-docs](https://docs.rs/repr_offset/badge.svg)](https://docs.rs/repr_offset/*)



`repr_offset` allows computing and safely using field offsets from types with a defined layout.

Currently only `#[repr(C)]`/`#[repr(C,packed)]`/`#[repr(C,align)]` structs are supported.

# Features

These are some of the features this library provides:

- The [`ReprOffset`] derive macro, which outputs associated constants with the
offsets of fields, and implements the [`GetFieldOffset`] trait for each field.<br>

- The [`FieldOffset`] type (how offsets are represented),
with methods for operating on a field through a pointer to the struct,
including getting a reference(or pointer) to the field.

- The [`unsafe_struct_field_offsets`] macro as an alternative to the
[`ReprOffset`] derive macro, most useful when the "derive" feature is disabled.

- The [`GetFieldOffset`] trait, for getting the [`FieldOffset`] for a field,
and the [`OFF!`], [`off`], [`PUB_OFF!`], and [`pub_off`] macros for
getting the [`FieldOffset`] for a field with a convenient syntax.

- The extension traits from the [`ext`] module,
which define methods for operating on a field, given a [`FieldOffset`].

# Examples 

For **examples** you can look at
[the examples section of the documentation for the root module of the `repr_offset` crate
](https://docs.rs/repr_offset/*/repr_offset/index.html#root-mod-examples)

# Future plans

None for now.

# Cargo features

These are the cargo features in `repr_offset`:

- `derive` (disabled by default): 
Enables the [`ReprOffset`] derive macro.
This requires the same Rust versions as `syn`, which is currently `>= 1.56.0`.

- `"for_examples"` (disabled by default): 
Enables the `for_examples` module, with types used in documentation examples.

Adding the "derive" feature to the Cargo.toml file:
```toml
repr_offset = { version = "0.2", features = ["derive"] }
```


# no-std support

This library is unconditionally `#![no_std]`,and that is unlikely to change in the future.

# Minimum Rust version

This crate support Rust back to 1.41.0.

# License

Licensed under the Zlib license

[`ReprOffset`]:
https://docs.rs/repr_offset/*/repr_offset/derive.ReprOffset.html

[`unsafe_struct_field_offsets`]:
https://docs.rs/repr_offset/*/repr_offset/macro.unsafe_struct_field_offsets.html

[`FieldOffset`]: 
https://docs.rs/repr_offset/*/repr_offset/struct.FieldOffset.html

[`OFF!`]: https://docs.rs/repr_offset/*/repr_offset/macro.OFF.html
[`off`]: https://docs.rs/repr_offset/*/repr_offset/macro.off.html
[`PUB_OFF!`]: https://docs.rs/repr_offset/*/repr_offset/macro.PUB_OFF.html
[`pub_off`]: https://docs.rs/repr_offset/*/repr_offset/macro.pub_off.html

[`GetFieldOffset`]:
https://docs.rs/repr_offset/*/repr_offset/get_field_offset/trait.GetFieldOffset.html

[`ext`]:
https://docs.rs/repr_offset/*/repr_offset/ext/index.html



