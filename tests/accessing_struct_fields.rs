use repr_offset::{
    _priv_run_with_types,
    types_for_tests::{
        Align16, Align4, StructAlign2, StructAlign4, StructAlign8, StructPacked, StructPacked16,
        StructPacked2, StructPacked4, StructPacked8, StructReprC,
    },
};

#[test]
fn access_aligned() {
    _priv_run_with_types! {
        type_constructors[ StructReprC, StructAlign2, StructAlign4, StructAlign8 ],
        (vec![0,1,2,3], Align16(5u8), 16.0_f64, [Align16(());0])
        |var, off0, off1, off2, off3| unsafe{
            assert_eq!( off0.get(&var), &vec![0,1,2,3] );
            assert_eq!( off0.get_raw(&var), off0.get(&var) as *const _ );
            assert_eq!( off0.get_mut(&mut var), &mut vec![0,1,2,3] );
            assert_eq!( &mut *off0.get_raw_mut(&mut var), &mut vec![0,1,2,3] );

            assert_eq!( off1.get(&var), &Align16(5u8) );
            assert_eq!( &*off1.get_raw(&var), off1.get(&var) );
            assert_eq!( off1.get_mut(&mut var), &mut Align16(5u8) );
            assert_eq!( &mut *off1.get_raw_mut(&mut var), &mut Align16(5u8) );
            assert_eq!( off1.get_copy_unaligned(&var), Align16(5u8) );
            assert_eq!( off1.get_copy(&var), Align16(5u8) );

            assert_eq!( off2.get(&var), &16.0 );
            assert_eq!( off2.get_raw(&var), off2.get(&var) as *const _ );
            assert_eq!( off2.get_mut(&mut var), &mut 16.0 );
            assert_eq!( &mut *off2.get_raw_mut(&mut var), &mut 16.0 );
            assert_eq!( off2.get_copy_unaligned(&var), 16.0 );
            assert_eq!( off2.get_copy(&var), 16.0 );

            assert_eq!( off3.get(&var), &[Align16(());0] );
            assert_eq!( off3.get_raw(&var), off3.get(&var) as *const _ );
            assert_eq!( off3.get_mut(&mut var), &mut [Align16(());0] );
            assert_eq!( &mut *off3.get_raw_mut(&mut var), &mut [Align16(());0] );
            assert_eq!( off3.get_copy_unaligned(&var), [Align16(());0] );
            assert_eq!( off3.get_copy(&var), [Align16(());0] );

        },
    }
}

trait EnsureUncallable: Sized {
    fn get<T>(self, _: &T) -> &'static str {
        "nope"
    }
    fn get_mut<T>(self, _: &mut T) -> &'static str {
        "nope"
    }
}

impl<This> EnsureUncallable for This {}

#[test]
fn access_unaligned() {
    _priv_run_with_types! {
        type_constructors[
            StructPacked, StructPacked2, StructPacked4, StructPacked8,StructPacked16,
        ],
        ([3usize, 5, 8], Align16(5u8), 16.0_f64, [Align4(());0])
        |var, off0, off1, off2, off3| unsafe{
            assert_eq!( off0.get(&var), "nope");
            assert_eq!( off0.get_mut(&mut var), "nope");
            assert_eq!( off0.get_raw(&var).read_unaligned(), [3usize, 5, 8] );
            assert_eq!( off0.get_raw_mut(&mut var).read_unaligned(), [3usize, 5, 8] );

            assert_eq!( off1.get(&var), "nope");
            assert_eq!( off1.get_mut(&mut var), "nope");
            assert_eq!( off1.get_raw(&var).read_unaligned(), Align16(5u8) );
            assert_eq!( off1.get_raw_mut(&mut var).read_unaligned(), Align16(5u8) );
            assert_eq!( off1.get_copy_unaligned(&var), Align16(5u8) );
            assert_eq!( off1.get_copy(&var), Align16(5u8) );

            assert_eq!( off2.get(&var), "nope");
            assert_eq!( off2.get_mut(&mut var), "nope");
            assert_eq!( off2.get_raw(&var).read_unaligned(), 16.0 );
            assert_eq!( off2.get_raw_mut(&mut var).read_unaligned(), 16.0 );
            assert_eq!( off2.get_copy_unaligned(&var), 16.0 );
            assert_eq!( off2.get_copy(&var), 16.0 );

            assert_eq!( off3.get(&var), "nope");
            assert_eq!( off3.get_mut(&mut var), "nope");
            assert_eq!( off3.get_raw(&var).read_unaligned(), [Align4(());0] );
            assert_eq!( off3.get_raw_mut(&mut var).read_unaligned(), [Align4(());0] );
            assert_eq!( off3.get_copy_unaligned(&var), [Align4(());0] );
            assert_eq!( off3.get_copy(&var), [Align4(());0] );

        },
    }
}
