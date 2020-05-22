use repr_offset::{
    types_for_tests::{StructReprC, Transparent},
    Aligned, FieldOffset, Packed,
};

type Consts = StructReprC<(), (u32, u32, u32, u32), (), ()>;

#[test]
fn cast_struct_method() {
    let this = Transparent(StructReprC {
        a: 3,
        b: 5,
        c: 8,
        d: 13,
    });

    type To = Transparent<StructReprC<u32, u32, u32, u32>>;

    unsafe {
        assert_eq!(Consts::OFFSET_A.cast_struct::<To>().get(&this), &3);
        assert_eq!(Consts::OFFSET_B.cast_struct::<To>().get(&this), &5);
        assert_eq!(Consts::OFFSET_C.cast_struct::<To>().get(&this), &8);
        assert_eq!(Consts::OFFSET_D.cast_struct::<To>().get(&this), &13);
    }
}

#[test]
fn cast_field_method() {
    let this = StructReprC {
        a: !3u32 + 1,
        b: !5u32 + 1,
        c: !8u32 + 1,
        d: !13u32 + 1,
    };

    unsafe {
        assert_eq!(Consts::OFFSET_A.cast_field::<i32>().get(&this), &-3);
        assert_eq!(Consts::OFFSET_B.cast_field::<i32>().get(&this), &-5);
        assert_eq!(Consts::OFFSET_C.cast_field::<i32>().get(&this), &-8);
        assert_eq!(Consts::OFFSET_D.cast_field::<i32>().get(&this), &-13);
    }
}

#[test]
fn cast_packing() {
    let this = StructReprC {
        a: !3u32 + 1,
        b: !5u32 + 1,
        c: !8u32 + 1,
        d: !13u32 + 1,
    };

    let _: FieldOffset<_, _, Aligned> = Consts::OFFSET_A;
    let _: FieldOffset<_, _, Aligned> = Consts::OFFSET_B;
    let _: FieldOffset<_, _, Aligned> = Consts::OFFSET_C;
    let _: FieldOffset<_, _, Aligned> = Consts::OFFSET_D;

    let packed_a: FieldOffset<_, _, Packed> = Consts::OFFSET_A.cast_packed();
    let packed_b: FieldOffset<_, _, Packed> = Consts::OFFSET_B.cast_packed();
    let packed_c: FieldOffset<_, _, Packed> = Consts::OFFSET_C.cast_packed();
    let packed_d: FieldOffset<_, _, Packed> = Consts::OFFSET_D.cast_packed();

    assert_eq!(packed_a.get_copy(&this), (-3i32) as u32);
    assert_eq!(packed_b.get_copy(&this), (-5i32) as u32);
    assert_eq!(packed_c.get_copy(&this), (-8i32) as u32);
    assert_eq!(packed_d.get_copy(&this), (-13i32) as u32);

    unsafe {
        let _: FieldOffset<_, _, Aligned> = packed_a.cast_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_b.cast_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_c.cast_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_d.cast_aligned();

        assert_eq!(packed_a.cast_aligned(), Consts::OFFSET_A);
        assert_eq!(packed_b.cast_aligned(), Consts::OFFSET_B);
        assert_eq!(packed_c.cast_aligned(), Consts::OFFSET_C);
        assert_eq!(packed_d.cast_aligned(), Consts::OFFSET_D);
    }
}
