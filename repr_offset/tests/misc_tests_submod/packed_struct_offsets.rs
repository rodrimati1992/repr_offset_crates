#[cfg(feature = "priv_raw_ref")]
#[test]
fn packed_struct_layouts() {
    repr_offset::_priv_for_permutations_test! {
        packing = packed,
        struct StructPacked;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = packed,
        struct StructPacked2;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = packed,
        struct StructPacked4;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = packed,
        struct StructPacked8;
    }
}
