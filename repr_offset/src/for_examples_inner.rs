#![allow(missing_docs)]

use crate::{Aligned, Unaligned};

macro_rules! declare_example_struct {
    (
        $(#[$meta:meta])*
        struct $name:ident;
        alignment =  $alignment:ty,
        $(impl_GetFieldOffset = $impl_gdo:ident,)?
    ) => {
        $(#[$meta])*
        #[derive(Default)]
        pub struct $name<A = (),B = (),C = (),D = ()>{
            pub a:A,
            pub b:B,
            pub c:C,
            pub d:D,
        }

        impl<A,B,C,D> Copy for $name<A,B,C,D>
        where
            A: Copy,
            B: Copy,
            C: Copy,
            D: Copy,
        {}

        impl<A,B,C,D> Clone for $name<A,B,C,D>
        where
            A: Copy,
            B: Copy,
            C: Copy,
            D: Copy,
        {
            fn clone(&self)->Self{
                *self
            }
        }

        unsafe_struct_field_offsets!{
            alignment =  $alignment,
            $(impl_GetFieldOffset = $impl_gdo,)?
            impl[A,B,C,D] $name<A,B,C,D>{
                /// The offset of the `a` field
                pub const OFFSET_A, a: A;
                /// The offset of the `b` field
                pub const OFFSET_B, b: B;
                /// The offset of the `c` field
                pub const OFFSET_C, c: C;
                /// The offset of the `d` field
                pub const OFFSET_D, d: D;
            }
        }
    };
}

declare_example_struct! {
    /// An example `#[repr(C)]` type
    #[repr(C)]
    struct ReprC;
    alignment = Aligned,
}

declare_example_struct! {
    /// An example `#[repr(C)]` type which doesn't implement [`GetFieldOffset`]
    ///
    /// [`GetFieldOffset`]: ../get_field_offset/trait.GetFieldOffset.html
    #[repr(C)]
    struct ReprCNoGFO;
    alignment = Aligned,
    impl_GetFieldOffset = false,
}

declare_example_struct! {
    /// An example `#[repr(C, align(4))]` type
    #[repr(C, align(4))]
    struct ReprAlign4;
    alignment = Aligned,
}

declare_example_struct! {
    /// An example `#[repr(C, packed)]` type
    #[repr(C, packed)]
    struct ReprPacked;
    alignment = Unaligned,
}

declare_example_struct! {
    /// An example `#[repr(C, packed(2))]` type.
    #[repr(C, packed(2))]
    struct ReprPacked2;
    alignment = Unaligned,
}
