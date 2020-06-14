#![cfg(feature = "priv_expensive_test")]

use repr_offset::{
    _priv_copy_tests as copy_tests, _priv_run_with_types, _priv_swap_tests as swap_tests,
    types_for_tests::{
        Align16, StructAlign2, StructAlign4, StructAlign8, StructPacked, StructReprC,
    },
    Aligned, FieldOffset, Unaligned,
};

#[test]
fn access_aligned() {
    _priv_run_with_types! {
        type_constructors[ StructReprC, StructAlign2, StructAlign4, StructAlign8 ],
        (vec![0, 1, 2, 3], Align16(5u8), 16.0_f64, [Align16(());0])
        (vec![5, 8, 13, 21], Align16(34u8), 100.0_f64, [Align16(());0])
        |var, other, off0, off1, off2, off3| {unsafe{
            assert_eq!( off0.get(&var), &vec![0, 1, 2, 3] );
            assert_eq!( off0.get_ptr(&var), off0.get(&var) as *const _ );
            assert_eq!( off0.raw_get(&var), off0.get(&var) as *const _ );
            assert_eq!( off0.wrapping_raw_get(&var), off0.get(&var) as *const _ );
            assert_eq!( off0.get_mut(&mut var), &mut vec![0, 1, 2, 3] );
            assert_eq!( &mut *off0.get_mut_ptr(&mut var), &mut vec![0, 1, 2, 3] );
            assert_eq!( &mut *off0.raw_get_mut(&mut var), &mut vec![0, 1, 2, 3] );
            assert_eq!( &mut *off0.wrapping_raw_get_mut(&mut var), &mut vec![0, 1, 2, 3] );
            {
                let mut tmp0 = off0.read(&var);
                assert_eq!( tmp0, vec![0, 1, 2, 3] );
                tmp0.push(13);
                off0.write(&mut var, tmp0);
                assert_eq!( off0.get(&var), &vec![0, 1, 2, 3, 13] );
                off0.replace_mut(&mut var, vec![0, 1, 2, 3]);
            }
            swap_tests!(
                off0,
                get_with = |off, v|{
                    let mut _x = off0; _x = off;
                    off.get(v).clone()
                },
                variables(var, other)
                values(vec![0, 1, 2, 3], vec![5, 8, 13, 21])
            );
            assert_eq!( off0.replace(&mut var, vec![100, 101, 102]), vec![0, 1, 2, 3] );
            assert_eq!( off0.replace(&mut var, vec![200, 201, 202]), vec![100, 101, 102] );
            assert_eq!( off0.replace_mut(&mut var, vec![300, 301, 302]), vec![200, 201, 202] );
            assert_eq!( off0.replace_mut(&mut var, vec![400, 401, 402]), vec![300, 301, 302] );

            assert_eq!( off1.get(&var), &Align16(5u8) );
            assert_eq!( &*off1.get_ptr(&var), off1.get(&var) );
            assert_eq!( &*off1.raw_get(&var), off1.get(&var) );
            assert_eq!( &*off1.wrapping_raw_get(&var), off1.get(&var) );
            assert_eq!( off1.get_mut(&mut var), &mut Align16(5u8) );
            assert_eq!( &mut *off1.get_mut_ptr(&mut var), &mut Align16(5u8) );
            assert_eq!( &mut *off1.raw_get_mut(&mut var), &mut Align16(5u8) );
            assert_eq!( &mut *off1.wrapping_raw_get_mut(&mut var), &mut Align16(5u8) );
            assert_eq!( off1.get_copy(&var), Align16(5u8) );
            assert_eq!( off1.read_copy(&var), Align16(5u8) );
            assert_eq!( off1.read(&var), Align16(5u8) );
            off1.write(&mut var, Align16(8u8));
            assert_eq!( off1.read(&var), Align16(8u8) );
            assert_eq!( off1.replace(&mut var, Align16(13u8)), Align16(8u8) );
            assert_eq!( off1.replace_mut(&mut var, Align16(21u8)), Align16(13u8) );
            assert_eq!( off1.read(&var), Align16(21u8) );
            swap_tests!(
                off1,
                get_with = FieldOffset::<_,_,Aligned>::get_copy,
                variables(var, other)
                values(Align16(21u8), Align16(34u8))
            );
            copy_tests!(
                off1,
                get_with = FieldOffset::<_,_,Aligned>::get_copy,
                variables(var, other)
                values(Align16(100u8), Align16(105u8), Align16(108u8))
            );

            assert_eq!( off2.get(&var), &16.0 );
            assert_eq!( off2.get_ptr(&var), off2.get(&var) as *const _ );
            assert_eq!( off2.raw_get(&var), off2.get(&var) as *const _ );
            assert_eq!( off2.wrapping_raw_get(&var), off2.get(&var) as *const _ );
            assert_eq!( off2.get_mut(&mut var), &mut 16.0 );
            assert_eq!( &mut *off2.get_mut_ptr(&mut var), &mut 16.0 );
            assert_eq!( &mut *off2.raw_get_mut(&mut var), &mut 16.0 );
            assert_eq!( &mut *off2.wrapping_raw_get_mut(&mut var), &mut 16.0 );
            assert_eq!( off2.get_copy(&var), 16.0 );
            assert_eq!( off2.read_copy(&var), 16.0 );
            assert_eq!( off2.read(&var), 16.0 );
            off2.write(&mut var, 24.0);
            assert_eq!( off2.read(&var), 24.0 );
            assert_eq!( off2.replace(&mut var, 25.0), 24.0 );
            assert_eq!( off2.replace_mut(&mut var, 26.0), 25.0 );
            assert_eq!( off2.read(&var), 26.0);
            swap_tests!(
                off2,
                get_with = FieldOffset::<_,_,Aligned>::get_copy,
                variables(var, other)
                values(26.0, 100.0)
            );
            copy_tests!(
                off2,
                get_with = FieldOffset::<_,_,Aligned>::get_copy,
                variables(var, other)
                values(103.0, 105.0, 108.0)
            );


            assert_eq!( off3.get(&var), &[Align16(());0] );
            assert_eq!( off3.get_ptr(&var), off3.get(&var) as *const _ );
            assert_eq!( off3.raw_get(&var), off3.get(&var) as *const _ );
            assert_eq!( off3.wrapping_raw_get(&var), off3.get(&var) as *const _ );
            assert_eq!( off3.get_mut(&mut var), &mut [Align16(());0] );
            assert_eq!( &mut *off3.get_mut_ptr(&mut var), &mut [Align16(());0] );
            assert_eq!( &mut *off3.raw_get_mut(&mut var), &mut [Align16(());0] );
            assert_eq!( &mut *off3.wrapping_raw_get_mut(&mut var), &mut [Align16(());0] );
            assert_eq!( off3.get_copy(&var), [Align16(());0] );
            assert_eq!( off3.read_copy(&var), [Align16(());0] );
        }}
    }

    type ReprCConsts = StructReprC<(), (u8, u16, u32, u64), (), ()>;
    type PackedConsts = StructPacked<(), (u8, u16, u32, u64), (), ()>;

    type ReprCType = StructReprC<u8, u16, u32, u64>;
    type PackedType = StructPacked<u8, u16, u32, u64>;

    const SECOND: ReprCType = StructReprC {
        a: 3,
        b: 5,
        c: 8,
        d: 13,
    };
    const FOURTH: PackedType = StructPacked {
        a: 21,
        b: 34,
        c: 55,
        d: 89,
    };
    _priv_run_with_types! {
        type_constructors[ StructReprC, StructAlign2, StructAlign4, StructAlign8 ],
        (0u32, SECOND, 0u32, FOURTH)
        (0u32, SECOND, 0u32, FOURTH)
        |var, _other, off0, off1, off2, off3| {
            let _: FieldOffset<_, ReprCType, Aligned> = off1;
            let _: FieldOffset<_, PackedType, Aligned> = off3;

            off0.get_copy(&var);
            off2.get_copy(&var);

            let off1_a: FieldOffset<_, u8, Aligned> = off1.add(ReprCConsts::OFFSET_A);
            let off1_b: FieldOffset<_, u16, Aligned> = off1.add(ReprCConsts::OFFSET_B);
            let off1_c: FieldOffset<_, u32, Aligned> = off1.add(ReprCConsts::OFFSET_C);
            let off1_d: FieldOffset<_, u64, Aligned> = off1.add(ReprCConsts::OFFSET_D);

            assert_eq!( off1_a.get_copy(&var), 3 );
            assert_eq!( off1_b.get_copy(&var), 5 );
            assert_eq!( off1_c.get_copy(&var), 8 );
            assert_eq!( off1_d.get_copy(&var), 13 );
            unsafe{
                assert_eq!( off1_a.read_copy(&var), 3 );
                assert_eq!( off1_b.read_copy(&var), 5 );
                assert_eq!( off1_c.read_copy(&var), 8 );
                assert_eq!( off1_d.read_copy(&var), 13 );
            }

            let off3_a: FieldOffset<_, u8, Unaligned> = off3.add(PackedConsts::OFFSET_A);
            let off3_b: FieldOffset<_, u16, Unaligned> = off3.add(PackedConsts::OFFSET_B);
            let off3_c: FieldOffset<_, u32, Unaligned> = off3.add(PackedConsts::OFFSET_C);
            let off3_d: FieldOffset<_, u64, Unaligned> = off3.add(PackedConsts::OFFSET_D);

            assert_eq!( off3_a.get_copy(&var), 21 );
            assert_eq!( off3_b.get_copy(&var), 34 );
            assert_eq!( off3_c.get_copy(&var), 55 );
            assert_eq!( off3_d.get_copy(&var), 89 );
            unsafe{
                assert_eq!( off3_a.read_copy(&var), 21 );
                assert_eq!( off3_b.read_copy(&var), 34 );
                assert_eq!( off3_c.read_copy(&var), 55 );
                assert_eq!( off3_d.read_copy(&var), 89 );
            }
        }
    }
}
