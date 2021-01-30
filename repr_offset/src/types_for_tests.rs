#![allow(missing_docs)]

use crate::{Aligned, Unaligned};

use core::{
    cmp::PartialEq,
    fmt::{self, Debug},
};

macro_rules! declare_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident;
        alignment =  $alignment:ty,
    ) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $name<A,B,C,D>
        where
            A: Default,
        {
            pub a:A,
            pub b:B,
            pub c:C,
            pub d:D,
        }

        impl<A,B,C,D> Copy for $name<A,B,C,D>
        where
            A: Default + Copy,
            B: Copy,
            C: Copy,
            D: Copy,
        {}

        impl<A,B,C,D> Clone for $name<A,B,C,D>
        where
            A: Default + Copy,
            B: Copy,
            C: Copy,
            D: Copy,
        {
            fn clone(&self)->Self{
                *self
            }
        }

        impl<A,B,C,D> Debug for $name<A,B,C,D>
        where
            A: Default + Copy + Debug,
            B: Copy + Debug,
            C: Copy + Debug,
            D: Copy + Debug,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
                let Self{a, b, c, d} = *self;
                f.debug_struct(stringify!($name))
                 .field("a",&a)
                 .field("b",&b)
                 .field("c",&c)
                 .field("d",&d)
                 .finish()
            }
        }

        impl<A,B,C,D> PartialEq for $name<A,B,C,D>
        where
            A: Default + Copy + PartialEq,
            B: Copy + PartialEq,
            C: Copy + PartialEq,
            D: Copy + PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                let Self{a: this_a, b: this_b, c: this_c, d: this_d} = *self;
                let Self{a: other_a, b: other_b, c: other_c, d: other_d} = *other;
                (this_a == other_a)&&(this_b == other_b)&&
                (this_c == other_c)&&(this_d == other_d)
            }
        }

        unsafe_struct_field_offsets!{
            Self = $name<A,B,C,D>,
            alignment =  $alignment,
            impl[A,B,C,D] $name<(),(A,B,C,D),(),()>
            where [ A: Default, ]
            {
                pub const OFFSET_A, a: A;
                pub const OFFSET_B, b: B;
                pub const OFFSET_C, c: C;
                pub const OFFSET_D, d: D;
            }
        }
    };
}

declare_struct! {
    #[repr(C)]
    struct StructReprC;
    alignment =  Aligned,
}

declare_struct! {
    #[repr(C,packed)]
    struct StructPacked;
    alignment =  Unaligned,
}

declare_struct! {
    #[repr(C,packed(2))]
    struct StructPacked2;
    alignment =  Unaligned,
}

declare_struct! {
    #[repr(C,packed(4))]
    struct StructPacked4;
    alignment =  Unaligned,
}

declare_struct! {
    #[repr(C,packed(8))]
    struct StructPacked8;
    alignment =  Unaligned,
}

declare_struct! {
    #[repr(C,packed(16))]
    struct StructPacked16;
    alignment =  Unaligned,
}

declare_struct! {
    #[repr(C,align(2))]
    struct StructAlign2;
    alignment =  Aligned,
}

declare_struct! {
    #[repr(C,align(4))]
    struct StructAlign4;
    alignment =  Aligned,
}

declare_struct! {
    #[repr(C,align(8))]
    struct StructAlign8;
    alignment =  Aligned,
}

#[repr(C, packed(16))]
#[derive(Default)]
pub struct Packed16<T>(pub T);

#[repr(C, packed(8))]
#[derive(Default)]
pub struct Packed8<T>(pub T);

#[repr(C, packed(4))]
#[derive(Default)]
pub struct Packed4<T>(pub T);

#[repr(C, packed)]
#[derive(Default)]
pub struct Packed2<T>(pub T);

#[repr(C, packed)]
#[derive(Default)]
pub struct Packed1<T>(pub T);

#[repr(align(16))]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Align16<T>(pub T);

#[repr(align(8))]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Align8<T>(pub T);

#[repr(align(4))]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Align4<T>(pub T);

#[repr(align(2))]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Align2<T>(pub T);

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Transparent<T>(pub T);
