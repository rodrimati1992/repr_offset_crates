#[test]
fn aligned_struct_layouts() {
    repr_offset::_priv_for_permutations_test! {
        packing = aligned,
        struct StructReprC;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = aligned,
        struct StructAlign2;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = aligned,
        struct StructAlign4;
    }
    repr_offset::_priv_for_permutations_test! {
        packing = aligned,
        struct StructAlign8;
    }
}
