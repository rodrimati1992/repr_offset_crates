This is the changelog,summarising changes in each version(some minor changes may be ommited).

# 0.2.0

- Bumped MSRV to Rust 1.41.0.

- Added `GetFieldOffset` trait in the new `get_field_offset` module,
to get the `FieldOffset` for a field,
implemented by the `unsafe_struct_field_offsets` and `ReprOffset` macros.

- Defined some more itens in `get_field_offset` module. The `FieldOffsetWithVis`,and `ImplGetNestedFieldOffset` structs;
`GetPubFieldOffset` alias for `GetFieldOFfset` with public fields;
`ImplsGetFieldOffset` marker trait for types tha implement `GetFieldOFfset`;
`FieldAlignment`, `FieldPrivacy`, `FieldType`, `PrivFieldAlignment`, and `PrivFieldType` type aliases, 

- Added `ext` module, with these extension traits for operating on fields, given the `FieldOffset` for the field: `ROExtAcc`, `ROExtOps`, `ROExtRawAcc`, `ROExtRawMutAcc`, `ROExtRawOps`, `ROExtRawMutOps`

- Added `OFF`, `PUB_OFF`, `off`, and `pub_off` macros for getting the field offset for a field.

- Added `privacy` module, with `IsPrivate`, and `IsPublic` structs; `CombinePrivacy`, and `PrivacyCombine` traits; `PrivacyOut`type alias.

- Added `impl_GetFieldOffset` parameter to the `unsafe_struct_field_offsets` macro,
to diasble the GetFieldOffset impls. Now the `Self = Foo,` parameter implies `impl_GetFieldOffset = false`.

- Added `#[roff(impl_GetFieldOffset = false)]` helper attribute to `ReprOffset` derive.

- Added tstr `0.2` as a dependency.

- Added these reexports in the root module:
`tstr` crate, all traits from `ext` module, get_field_offset::{GetFieldOffset, FieldType}.

- Moved the docs for `ReprOffset` to the reexport.

- Made the `alignment` module (where `Aligned` and `Unaligned` are declared) public

- Renamed `CombinePacking` and `CombinePackingOut` to `CombineAlignment` and `CombineAlignmentOut`.

- Removed reexports of `Alignment`, `CombineAlignment`, and `CombineAlignmentOut`.

- Added impls of `CombineAlignment` for tuples of `Aligned` and `Unaligned` permutations

- Added `utils::MakePhantomData` helper struct, `utils::PointerTarget` trait.

- Added `for_examples::ReprCNoGFO` for an example type that doesn't implement `GetFieldOffset`

- Added `FieldOffset::identity` constructor


# 0.1.2

Changed CI to use github actions, updating readme to reflect that.

# 0.1.1

Worked around compilation error in beta and nighly channels by updating the minimum version for `as_derive_utils` dependency.

# 0.1.0

- Created the `repr_offset` crate and `repr_offset_derive` proc macro crate.

- Defined the `FieldOffset` struct,a strongly typed field offset,
with many methods that take pointers(including references) to structs and 
operate on the field that the `FieldOffset` is an offset for.

- Defined the `Aligned` and `Unaligned` types,
which represent the alignment of a field inside of a type.

- Defined `Alignment` marker trait for the `Aligned` and `Unaligned` types.

- Defined the `CombinePacking` trait and CombinePackingOut type alias to
combine all permutations of `Aligned` and `Unaligned` on the type level.

- Defined the `unsafe_struct_field_offsets` macro for declaring field offset associated constants.

- Defined the `ReprOffset` derive macro to declare field offset associated constants,
it's defined in `repr_offset_derive`,and re-exported (and documented) in `repr_offset`.

- Defined the documentation for the `ReprOffset` macro in the 
`repr_offset::docs::repr_offset_macro` module.

- Defined the `offset_calc` module with functions for calculating the offsets of fields.

- Defined the `for_examples` module (only enabled in docs and tests),
requiring the "for_examples" feature to enable it.

- Defined the "derive" feature to re-export the `ReprOffset` derive macro in `repr_offset`,
enabled by default. When disabled `repr_offset` compiles in very little time.

- Added a build script to `repr_offset`, which enables some features used by documentation.

- Added as build dependency: `rustc_version`.

- Added as dependencies of `repr_offset_derive`:
`core_extensions`, `as_derive_utils`, `proc-macro2`, `quote`,and `syn` .