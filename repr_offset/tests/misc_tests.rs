#![cfg_attr(feature = "priv_raw_ref", feature(raw_ref_op))]

mod misc_tests_submod {
    mod accessing_struct_fields;
    //mod accessing_unaligned_struct_fields;
    mod aligned_struct_offsets;
    mod derive_macro;
    mod from_examples;
    mod misc_fieldoffsets_methods;
    mod packed_struct_offsets;
    mod struct_field_offsets_macro;
}
