use repr_offset::{Aligned, FieldOffset, ReprOffset, Unaligned};

use std::{fmt::Debug, marker::PhantomData};

macro_rules! repeated_tests {
    (
        modules[ $($mod:ident),* $(,)? ]
    ) => {
        $({
            use $mod::{MStruct, Struct};
            assert_eq!( Struct::OFFSET_X, MStruct::OFFSET_X );
            assert_eq!( Struct::OFFSET_Y, MStruct::OFFSET_Y );
            assert_eq!( Struct::OFFSET_Z, MStruct::OFFSET_Z );
        })*
    };
}

#[test]
fn derive_vs_manual() {
    repeated_tests! {
        modules[
            repr_c,
            aligned,
            packed,
            packed_4,
            use_usize_offsets,
            starting_offset_a,
            starting_offset_b,
        ]
    }
}

mod repr_c {
    use super::*;

    #[repr(C)]
    #[derive(ReprOffset)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment =  Aligned,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod repr_c_tuple {
    use super::*;

    #[repr(C)]
    #[derive(ReprOffset)]
    pub struct Struct(
        pub u8,
        pub i8,
        #[roff(offset = "OFF_TWO")] pub u64,
        #[roff(offset_prefix = "OFF_")] pub &'static str,
    );

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment =  Aligned,

        impl[] MStruct {
            pub const OFFSET_0: u8;
            pub const OFFSET_1: i8;
            pub const OFF_TWO: u64;
            pub const OFF_3: &'static str;
        }
    }

    #[test]
    fn derive_tuple() {
        assert_eq!(Struct::OFFSET_0, MStruct::OFFSET_0);
        assert_eq!(Struct::OFFSET_1, MStruct::OFFSET_1);
        assert_eq!(Struct::OFF_TWO, MStruct::OFF_TWO);
        assert_eq!(Struct::OFF_3, MStruct::OFF_3);
    }
}

mod aligned {
    use super::*;

    #[repr(C, align(32))]
    #[derive(ReprOffset)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Aligned,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod packed {
    use super::*;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Unaligned,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod packed_4 {
    use super::*;

    #[repr(C, packed(4))]
    #[derive(ReprOffset)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Unaligned,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod use_usize_offsets {
    use super::*;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    #[roff(usize_offsets)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Unaligned,
        usize_offsets = true,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod starting_offset_a {
    use super::*;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    #[roff(unsafe_starting_offset = 100)]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Unaligned,
        starting_offset = 100,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod starting_offset_b {
    use super::*;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    #[roff(unsafe_starting_offset = "30 + 70")]
    pub struct Struct {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Unaligned,
        starting_offset = 100,

        impl[] MStruct {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }
}

mod starting_offset_c {
    use super::*;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    #[roff(unsafe_starting_offset = "std::mem::size_of::<T>()")]
    pub struct Struct<T> {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
        _marker: PhantomData<fn() -> T>,
    }

    pub struct MStruct<T>(T);

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct<T>,
        alignment = Unaligned,
        starting_offset = std::mem::size_of::<T>(),

        impl[T] MStruct<T> {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }

    macro_rules! test_type {
        ($type:ty) => {{
            type This = Struct<$type>;
            type Other = MStruct<$type>;
            const S: usize = std::mem::size_of::<$type>();
            assert_eq!(This::OFFSET_X, Other::OFFSET_X);
            assert_eq!(This::OFFSET_Y, Other::OFFSET_Y);
            assert_eq!(This::OFFSET_Z, Other::OFFSET_Z);

            assert_eq!(This::OFFSET_X.offset(), S);
            assert_eq!(This::OFFSET_Y.offset(), S + 1);
            assert_eq!(This::OFFSET_Z.offset(), S + 9);
        }};
    }

    #[test]
    fn generic_starting_offset_test() {
        test_type!(String);
        test_type!(u8);
        test_type!(u16);
    }
}

mod changed_names {
    use super::*;

    #[repr(C)]
    #[derive(ReprOffset)]
    #[roff(offset_prefix = "OFF_")]
    pub struct Struct {
        pub a: u8,
        pub b: u64,
        #[roff(offset_prefix = "OH_")]
        pub c: &'static str,
        #[roff(offset = "D_OFF")]
        pub d: bool,
    }

    pub struct MStruct;

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct,
        alignment = Aligned,

        impl[] MStruct {
            pub const OFFSET_A: u8;
            pub const OFFSET_B: u64;
            pub const OFFSET_C: &'static str;
            pub const OFFSET_D: bool;
        }
    }

    #[test]
    fn rename_derive_test() {
        assert_eq!(Struct::OFF_A, MStruct::OFFSET_A);
        assert_eq!(Struct::OFF_B, MStruct::OFFSET_B);
        assert_eq!(Struct::OH_C, MStruct::OFFSET_C);
        assert_eq!(Struct::D_OFF, MStruct::OFFSET_D);
    }
}

mod generic_params {
    use super::*;

    #[repr(C)]
    #[derive(ReprOffset)]
    pub struct Struct<'a, T: Copy>
    where
        T: Debug,
    {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
        _marker: PhantomData<(&'a (), T)>,
    }

    pub struct MStruct<'a, T>(PhantomData<(&'a (), T)>);

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct<'a,T>,
        alignment =  Aligned,

        impl['a, T] MStruct<'a, T>
        where[
            T: Copy + Debug,
        ] {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }

    fn helper<'a, T: Copy + Debug>() {
        assert_eq!(Struct::<'a, T>::OFFSET_X, MStruct::<'a, T>::OFFSET_X);
        assert_eq!(Struct::<'a, T>::OFFSET_Y, MStruct::<'a, T>::OFFSET_Y);
        assert_eq!(Struct::<'a, T>::OFFSET_Z, MStruct::<'a, T>::OFFSET_Z);
    }

    #[test]
    fn derive_generics_test() {
        helper::<u128>();
        helper::<u8>();
        helper::<()>();
    }
}

mod with_bounds {
    use super::*;

    #[repr(C)]
    #[derive(ReprOffset)]
    #[roff(bound = "T: Copy")]
    pub struct Struct<T> {
        pub x: u8,
        pub y: u64,
        pub z: &'static str,
        _marker: PhantomData<T>,
    }

    pub struct MStruct<T>(T);

    repr_offset::unsafe_struct_field_offsets! {
        Self = Struct<T>,
        alignment =  Aligned,

        impl[T] MStruct<T>
        where[ T: Copy ]
        {
            pub const OFFSET_X: u8;
            pub const OFFSET_Y: u64;
            pub const OFFSET_Z: &'static str;
        }
    }

    trait Foo {
        const OFFSET_X: &'static str = "X";
        const OFFSET_Y: &'static str = "Y";
        const OFFSET_Z: &'static str = "Z";
    }

    impl<T> Foo for T {}

    fn does_not_have_offsets<T>() {
        assert_eq!(Struct::<T>::OFFSET_X, MStruct::<T>::OFFSET_X);
        assert_eq!(Struct::<T>::OFFSET_Y, MStruct::<T>::OFFSET_Y);
        assert_eq!(Struct::<T>::OFFSET_Z, MStruct::<T>::OFFSET_Z);
        assert_eq!(Struct::<T>::OFFSET_X, "X");
        assert_eq!(Struct::<T>::OFFSET_Y, "Y");
        assert_eq!(Struct::<T>::OFFSET_Z, "Z");
    }
    fn has_offsets<T: Copy>() {
        assert_eq!(Struct::<T>::OFFSET_X, MStruct::<T>::OFFSET_X);
        assert_eq!(Struct::<T>::OFFSET_Y, MStruct::<T>::OFFSET_Y);
        assert_eq!(Struct::<T>::OFFSET_Z, MStruct::<T>::OFFSET_Z);
        let _: FieldOffset<_, u8, _> = Struct::<T>::OFFSET_X;
        let _: FieldOffset<_, u64, _> = Struct::<T>::OFFSET_Y;
        let _: FieldOffset<_, &'static str, _> = Struct::<T>::OFFSET_Z;
    }

    #[test]
    fn with_bounds_test() {
        has_offsets::<u8>();
        has_offsets::<u16>();
        does_not_have_offsets::<u8>();
        does_not_have_offsets::<u16>();
    }
}

mod privacy {
    use super::*;

    mod inner {
        use super::*;

        #[repr(C)]
        #[derive(ReprOffset)]
        pub struct Struct {
            pub x: u8,
            y: u64,
            z: &'static str,
        }
    }
    use self::inner::Struct;

    trait Foo {
        const OFFSET_Y: &'static str = "Y";
        const OFFSET_Z: &'static str = "Z";
    }

    impl<T> Foo for T {}

    #[test]
    fn derive_privacy_test() {
        assert_eq!(Struct::OFFSET_X.offset(), 0);
        assert_eq!(Struct::OFFSET_Y, "Y");
        assert_eq!(Struct::OFFSET_Z, "Z");
    }
}
