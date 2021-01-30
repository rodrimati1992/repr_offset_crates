use repr_offset::{
    alignment::{Aligned, Unaligned},
    get_field_offset::{FieldPrivacy, PrivFieldAlignment, PrivFieldType},
    privacy::{IsPrivate, IsPublic},
    tstr::alias,
    unsafe_struct_field_offsets,
};

#[derive(Default)]
#[repr(C)]
pub struct AlignedStruct<A, B, C, D> {
    a: A,
    b: B,
    c: C,
    d: D,
}

unsafe_struct_field_offsets! {
    alignment =  Aligned,

    impl[A,B,C,D] AlignedStruct<A,B,C,D>{
        const OFFSET_A, a: A;
        pub(super) const OFFSET_B, b: B;
        pub(crate) const OFFSET_C, c: C;
        pub const OFFSET_D, d: D;
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
#[repr(C, packed)]
pub struct PackedStruct<A, B, C, D> {
    a: A,
    b: B,
    c: C,
    d: D,
}

impl<'a, A, B, C, D> PackedStruct<&'a A, &'a B, &'a C, &'a D> {
    pub fn b((a, b, c, d): &'a (A, B, C, D)) -> Self {
        Self { a, b, c, d }
    }
}

unsafe_struct_field_offsets! {
    alignment =  Unaligned,

    impl[A,B,C,D] PackedStruct<A,B,C,D>{
        const OFFSET_A, a: A;
        pub(super) const OFFSET_B, b: B;
        pub(crate) const OFFSET_C, c: C;
        pub const OFFSET_D, d: D;
    }
}

////////////////////////////////////////////////////////////////////////////////

alias! {
    SA = a;
    SB = b;
    SC = c;
    SD = d;
    SAA = (a,a);
    SAB = (a,b);
    SAC = (a,c);
    SAD = (a,d);
    SBA = (b,a);
    SBB = (b,b);
    SBC = (b,c);
    SBD = (b,d);
    SCA = (c,a);
    SCB = (c,b);
    SCC = (c,c);
    SCD = (c,d);
    SDA = (d,a);
    SDB = (d,b);
    SDC = (d,c);
    SDD = (d,d);
}

////////////////////////////////////////////////////////////////////////////////

pub type FieldP<S, FN> = (
    PrivFieldType<S, FN>,
    PrivFieldAlignment<S, FN>,
    FieldPrivacy<S, FN>,
);

pub type AlignedInnerAA = AlignedStruct<u8, u16, u32, u64>;
pub type AlignedInnerBB = AlignedStruct<char, Option<char>, Option<bool>, bool>;
pub type AlignedInnerCC = AlignedStruct<i8, i16, i32, i64>;
pub type AlignedInnerDD = AlignedStruct<Option<u8>, Option<u16>, Option<u32>, Option<u64>>;

#[test]
fn struct_aligned_aligned() {
    pub type S = AlignedStruct<AlignedInnerAA, AlignedInnerBB, AlignedInnerCC, AlignedInnerDD>;

    let _: FieldP<S, SA> = (AlignedInnerAA::default(), Aligned, IsPrivate);
    let _: FieldP<S, SB> = (AlignedInnerBB::default(), Aligned, IsPrivate);
    let _: FieldP<S, SC> = (AlignedInnerCC::default(), Aligned, IsPrivate);
    let _: FieldP<S, SD> = (AlignedInnerDD::default(), Aligned, IsPublic);

    let _: FieldP<S, SAA> = (0u8, Aligned, IsPrivate);
    let _: FieldP<S, SAB> = (0u16, Aligned, IsPrivate);
    let _: FieldP<S, SAC> = (0u32, Aligned, IsPrivate);
    let _: FieldP<S, SAD> = (0u64, Aligned, IsPrivate);

    let _: FieldP<S, SBA> = (' ', Aligned, IsPrivate);
    let _: FieldP<S, SBB> = (Some(' '), Aligned, IsPrivate);
    let _: FieldP<S, SBC> = (Some(false), Aligned, IsPrivate);
    let _: FieldP<S, SBD> = (false, Aligned, IsPrivate);

    let _: FieldP<S, SCA> = (0i8, Aligned, IsPrivate);
    let _: FieldP<S, SCB> = (0i16, Aligned, IsPrivate);
    let _: FieldP<S, SCC> = (0i32, Aligned, IsPrivate);
    let _: FieldP<S, SCD> = (0i64, Aligned, IsPrivate);

    let _: FieldP<S, SDA> = (Some(0u8), Aligned, IsPrivate);
    let _: FieldP<S, SDB> = (Some(0u16), Aligned, IsPrivate);
    let _: FieldP<S, SDC> = (Some(0u32), Aligned, IsPrivate);
    let _: FieldP<S, SDD> = (Some(0u64), Aligned, IsPublic);
}

#[test]
fn struct_packed_aligned() {
    pub type S = PackedStruct<AlignedInnerAA, AlignedInnerBB, AlignedInnerCC, AlignedInnerDD>;

    let _: FieldP<S, SA> = (AlignedInnerAA::default(), Unaligned, IsPrivate);
    let _: FieldP<S, SB> = (AlignedInnerBB::default(), Unaligned, IsPrivate);
    let _: FieldP<S, SC> = (AlignedInnerCC::default(), Unaligned, IsPrivate);
    let _: FieldP<S, SD> = (AlignedInnerDD::default(), Unaligned, IsPublic);

    let _: FieldP<S, SAA> = (0u8, Unaligned, IsPrivate);
    let _: FieldP<S, SAB> = (0u16, Unaligned, IsPrivate);
    let _: FieldP<S, SAC> = (0u32, Unaligned, IsPrivate);
    let _: FieldP<S, SAD> = (0u64, Unaligned, IsPrivate);

    let _: FieldP<S, SBA> = (' ', Unaligned, IsPrivate);
    let _: FieldP<S, SBB> = (Some(' '), Unaligned, IsPrivate);
    let _: FieldP<S, SBC> = (Some(false), Unaligned, IsPrivate);
    let _: FieldP<S, SBD> = (false, Unaligned, IsPrivate);

    let _: FieldP<S, SCA> = (0i8, Unaligned, IsPrivate);
    let _: FieldP<S, SCB> = (0i16, Unaligned, IsPrivate);
    let _: FieldP<S, SCC> = (0i32, Unaligned, IsPrivate);
    let _: FieldP<S, SCD> = (0i64, Unaligned, IsPrivate);

    let _: FieldP<S, SDA> = (Some(0u8), Unaligned, IsPrivate);
    let _: FieldP<S, SDB> = (Some(0u16), Unaligned, IsPrivate);
    let _: FieldP<S, SDC> = (Some(0u32), Unaligned, IsPrivate);
    let _: FieldP<S, SDD> = (Some(0u64), Unaligned, IsPublic);
}

pub type PackedInnerAA = PackedStruct<Option<i8>, Option<i16>, Option<i32>, Option<i64>>;
pub type PackedInnerBB<'a> = PackedStruct<&'a char, &'a bool, &'a u128, &'a i128>;
pub type PackedInnerCC<'a> = PackedStruct<&'a i8, &'a i16, &'a i32, &'a i64>;
pub type PackedInnerDD<'a> = PackedStruct<&'a u8, &'a u16, &'a u32, &'a u64>;

#[test]
fn struct_aligned_packed() {
    pub type S<'a> =
        AlignedStruct<PackedInnerAA, PackedInnerBB<'a>, PackedInnerCC<'a>, PackedInnerDD<'a>>;

    let _: FieldP<S<'_>, SA> = (PackedInnerAA::default(), Aligned, IsPrivate);
    let _: FieldP<S<'_>, SB> = (
        PackedStruct::b(&(' ', true, 0u128, 0i128)),
        Aligned,
        IsPrivate,
    );
    let _: FieldP<S<'_>, SC> = (
        PackedStruct::b(&(0i8, 0i16, 0i32, 0i64)),
        Aligned,
        IsPrivate,
    );
    let _: FieldP<S<'_>, SD> = (PackedStruct::b(&(0u8, 0u16, 0u32, 0u64)), Aligned, IsPublic);

    let _: FieldP<S<'_>, SAA> = (Some(0i8), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAB> = (Some(0i16), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAC> = (Some(0i32), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAD> = (Some(0i64), Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SBA> = (&' ', Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBB> = (&true, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBC> = (&0u128, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBD> = (&0i128, Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SCA> = (&0i8, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCB> = (&0i16, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCC> = (&0i32, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCD> = (&0i64, Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SDA> = (&0u8, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDB> = (&0u16, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDC> = (&0u32, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDD> = (&0u64, Unaligned, IsPublic);
}

#[test]
fn struct_packed_packed() {
    pub type S<'a> =
        PackedStruct<PackedInnerAA, PackedInnerBB<'a>, PackedInnerCC<'a>, PackedInnerDD<'a>>;

    let _: FieldP<S<'_>, SA> = (PackedInnerAA::default(), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SB> = (
        PackedStruct::b(&(' ', true, 0u128, 0i128)),
        Unaligned,
        IsPrivate,
    );
    let _: FieldP<S<'_>, SC> = (
        PackedStruct::b(&(0i8, 0i16, 0i32, 0i64)),
        Unaligned,
        IsPrivate,
    );
    let _: FieldP<S<'_>, SD> = (
        PackedStruct::b(&(0u8, 0u16, 0u32, 0u64)),
        Unaligned,
        IsPublic,
    );

    let _: FieldP<S<'_>, SAA> = (Some(0i8), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAB> = (Some(0i16), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAC> = (Some(0i32), Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SAD> = (Some(0i64), Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SBA> = (&' ', Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBB> = (&true, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBC> = (&0u128, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SBD> = (&0i128, Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SCA> = (&0i8, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCB> = (&0i16, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCC> = (&0i32, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SCD> = (&0i64, Unaligned, IsPrivate);

    let _: FieldP<S<'_>, SDA> = (&0u8, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDB> = (&0u16, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDC> = (&0u32, Unaligned, IsPrivate);
    let _: FieldP<S<'_>, SDD> = (&0u64, Unaligned, IsPublic);
}
