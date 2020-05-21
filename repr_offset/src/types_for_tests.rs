use crate::{Aligned, Packed};

macro_rules! declare_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident;
        packing = $packing:ty,
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


        unsafe_struct_field_offsets!{
            Self = $name<A,B,C,D>,
            packing = $packing,
            impl[A,B,C,D] $name<(),(A,B,C,D),(),()>
            where [ A: Default, ]
            {
                pub const OFFSET_A: A;
                pub const OFFSET_B: B;
                pub const OFFSET_C: C;
                pub const OFFSET_D: D;
            }
        }
    };
}

declare_struct! {
    #[repr(C)]
    struct StructReprC;
    packing = Aligned,
}

declare_struct! {
    #[repr(C,packed)]
    struct StructPacked;
    packing = Packed,
}

declare_struct! {
    #[repr(C,packed(2))]
    struct StructPacked2;
    packing = Packed,
}

declare_struct! {
    #[repr(C,packed(4))]
    struct StructPacked4;
    packing = Packed,
}

declare_struct! {
    #[repr(C,packed(8))]
    struct StructPacked8;
    packing = Packed,
}

declare_struct! {
    #[repr(C,packed(16))]
    struct StructPacked16;
    packing = Packed,
}

declare_struct! {
    #[repr(C,align(2))]
    struct StructAlign2;
    packing = Aligned,
}

declare_struct! {
    #[repr(C,align(4))]
    struct StructAlign4;
    packing = Aligned,
}

declare_struct! {
    #[repr(C,align(8))]
    struct StructAlign8;
    packing = Aligned,
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
