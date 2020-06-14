use repr_offset::{
    types_for_tests::{StructPacked, StructReprC, Transparent},
    Aligned, FieldOffset, Unaligned,
};

type Consts = StructReprC<(), (u32, u32, u32, u32), (), ()>;

#[test]
#[allow(non_camel_case_types)]
fn add_method() {
    type ReprC_ReprC_C = StructReprC<(), (u8, ReprC_T, u32, u64), (), ()>;
    type ReprC_Packd_C = StructReprC<(), (u8, Packd_T, u32, u64), (), ()>;
    type Packd_ReprC_C = StructPacked<(), (u8, ReprC_T, u32, u64), (), ()>;
    type Packd_Packd_C = StructPacked<(), (u8, Packd_T, u32, u64), (), ()>;

    type ReprC_ReprC_T = StructReprC<u8, ReprC_T, u32, u64>;
    type ReprC_Packd_T = StructReprC<u8, Packd_T, u32, u64>;
    type Packd_ReprC_T = StructPacked<u8, ReprC_T, u32, u64>;
    type Packd_Packd_T = StructPacked<u8, Packd_T, u32, u64>;

    type ReprC_C = StructReprC<(), (u8, u16, u32, u64), (), ()>;
    type Packd_C = StructPacked<(), (i8, i16, i32, i64), (), ()>;

    type ReprC_T = StructReprC<u8, u16, u32, u64>;
    type Packd_T = StructPacked<i8, i16, i32, i64>;

    let vcc: ReprC_ReprC_T = StructReprC {
        a: 0,
        b: StructReprC {
            a: 3,
            b: 5,
            c: 8,
            d: 13,
        },
        c: 0,
        d: 0,
    };

    let vcp: ReprC_Packd_T = StructReprC {
        a: 0,
        b: StructPacked {
            a: 6,
            b: 10,
            c: 16,
            d: 26,
        },
        c: 0,
        d: 0,
    };

    let vpc: Packd_ReprC_T = StructPacked {
        a: 0,
        b: StructReprC {
            a: 9,
            b: 15,
            c: 24,
            d: 39,
        },
        c: 0,
        d: 0,
    };

    let vpp: Packd_Packd_T = StructPacked {
        a: 0,
        b: StructPacked {
            a: 12,
            b: 20,
            c: 32,
            d: 52,
        },
        c: 0,
        d: 0,
    };

    macro_rules! test_adds {
        (
            $(
                ($left:expr, $right:expr, ($($type_params:tt)*), $var:ident, $field_value:expr)
            )*
        ) => (
            $({
                let offa: FieldOffset<$($type_params)*> = $left.add($right);
                let offb: FieldOffset<$($type_params)*> = $left + $right;
                assert_eq!( offa.get_copy(&$var), $field_value );
                assert_eq!( offb.get_copy(&$var), $field_value );
            })*
        )
    }

    test_adds! {
        (ReprC_ReprC_C::OFFSET_B, ReprC_C::OFFSET_A, (ReprC_ReprC_T, u8, Aligned) , vcc, 3 )
        (ReprC_ReprC_C::OFFSET_B, ReprC_C::OFFSET_B, (ReprC_ReprC_T, u16, Aligned), vcc, 5 )
        (ReprC_ReprC_C::OFFSET_B, ReprC_C::OFFSET_C, (ReprC_ReprC_T, u32, Aligned), vcc, 8 )
        (ReprC_ReprC_C::OFFSET_B, ReprC_C::OFFSET_D, (ReprC_ReprC_T, u64, Aligned), vcc, 13 )

        (ReprC_Packd_C::OFFSET_B, Packd_C::OFFSET_A, (ReprC_Packd_T, i8, Unaligned) , vcp, 6  )
        (ReprC_Packd_C::OFFSET_B, Packd_C::OFFSET_B, (ReprC_Packd_T, i16, Unaligned), vcp, 10 )
        (ReprC_Packd_C::OFFSET_B, Packd_C::OFFSET_C, (ReprC_Packd_T, i32, Unaligned), vcp, 16 )
        (ReprC_Packd_C::OFFSET_B, Packd_C::OFFSET_D, (ReprC_Packd_T, i64, Unaligned), vcp, 26 )

        (Packd_ReprC_C::OFFSET_B, ReprC_C::OFFSET_A, (Packd_ReprC_T, u8, Unaligned) , vpc, 9  )
        (Packd_ReprC_C::OFFSET_B, ReprC_C::OFFSET_B, (Packd_ReprC_T, u16, Unaligned), vpc, 15 )
        (Packd_ReprC_C::OFFSET_B, ReprC_C::OFFSET_C, (Packd_ReprC_T, u32, Unaligned), vpc, 24 )
        (Packd_ReprC_C::OFFSET_B, ReprC_C::OFFSET_D, (Packd_ReprC_T, u64, Unaligned), vpc, 39 )

        (Packd_Packd_C::OFFSET_B, Packd_C::OFFSET_A, (Packd_Packd_T, i8, Unaligned) , vpp, 12 )
        (Packd_Packd_C::OFFSET_B, Packd_C::OFFSET_B, (Packd_Packd_T, i16, Unaligned), vpp, 20 )
        (Packd_Packd_C::OFFSET_B, Packd_C::OFFSET_C, (Packd_Packd_T, i32, Unaligned), vpp, 32 )
        (Packd_Packd_C::OFFSET_B, Packd_C::OFFSET_D, (Packd_Packd_T, i64, Unaligned), vpp, 52 )

    }
}

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
fn cast_alignment() {
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

    let packed_a: FieldOffset<_, _, Unaligned> = Consts::OFFSET_A.to_unaligned();
    let packed_b: FieldOffset<_, _, Unaligned> = Consts::OFFSET_B.to_unaligned();
    let packed_c: FieldOffset<_, _, Unaligned> = Consts::OFFSET_C.to_unaligned();
    let packed_d: FieldOffset<_, _, Unaligned> = Consts::OFFSET_D.to_unaligned();

    assert_eq!(packed_a.get_copy(&this), (-3i32) as u32);
    assert_eq!(packed_b.get_copy(&this), (-5i32) as u32);
    assert_eq!(packed_c.get_copy(&this), (-8i32) as u32);
    assert_eq!(packed_d.get_copy(&this), (-13i32) as u32);

    unsafe {
        let _: FieldOffset<_, _, Aligned> = packed_a.to_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_b.to_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_c.to_aligned();
        let _: FieldOffset<_, _, Aligned> = packed_d.to_aligned();

        assert_eq!(packed_a.to_aligned(), Consts::OFFSET_A);
        assert_eq!(packed_b.to_aligned(), Consts::OFFSET_B);
        assert_eq!(packed_c.to_aligned(), Consts::OFFSET_C);
        assert_eq!(packed_d.to_aligned(), Consts::OFFSET_D);
    }
}
