use repr_offset::{unsafe_struct_field_offsets, Aligned};

#[repr(C)]
pub struct Foo {
    pub foo: u8,
    pub bar: u64,
    pub baz: u16,
}

pub struct Consts;
pub struct UsizeConsts;
pub struct OffsetConsts;
pub struct AttributeConsts;

unsafe_struct_field_offsets! {
    alignment =  Aligned,

    impl[] Foo{
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;
        pub const OFFSET_BAZ: u16;
    }
}

unsafe_struct_field_offsets! {
    Self = Foo,
    alignment =  Aligned,
    usize_offsets = false,
    starting_offset = 0,

    impl[] Consts {
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;
        pub const OFFSET_BAZ: u16;
    }
}

unsafe_struct_field_offsets! {
    Self = Foo,
    alignment =  Aligned,
    usize_offsets = true,
    starting_offset = 0,

    impl[] UsizeConsts {
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;
        pub const OFFSET_BAZ: u16;
    }
}

unsafe_struct_field_offsets! {
    Self = Foo,
    alignment =  Aligned,
    usize_offsets = false,
    starting_offset = 96,

    impl[] OffsetConsts {
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;
        pub const OFFSET_BAZ: u16;
    }
}

unsafe_struct_field_offsets! {
    Self = Foo,
    alignment =  Aligned,

    #[cfg(FALSE)]
    impl[] AttributeConsts {
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;
        pub const OFFSET_BAZ: u16;
    }
}

unsafe_struct_field_offsets! {
    Self = Foo,
    alignment =  Aligned,

    impl[] AttributeConsts {
        pub const OFFSET_FOO: u8;
        pub const OFFSET_BAR: u64;

        #[cfg(FALSE)]
        pub const OFFSET_BAZ: u16;
    }
}

impl AttributeConsts {
    // This tests that the `OFFSET_BAZ` declared above was cfg-ed out.
    pub const OFFSET_BAZ: &'static str = "nope";
}

#[test]
fn offsets_macro_params() {
    assert_eq!(Foo::OFFSET_FOO, Consts::OFFSET_FOO);
    assert_eq!(Foo::OFFSET_BAR, Consts::OFFSET_BAR);
    assert_eq!(Foo::OFFSET_BAZ, Consts::OFFSET_BAZ);

    assert_eq!(Foo::OFFSET_FOO.offset(), UsizeConsts::OFFSET_FOO);
    assert_eq!(Foo::OFFSET_BAR.offset(), UsizeConsts::OFFSET_BAR);
    assert_eq!(Foo::OFFSET_BAZ.offset(), UsizeConsts::OFFSET_BAZ);

    assert_eq!(
        UsizeConsts::OFFSET_FOO + 96,
        OffsetConsts::OFFSET_FOO.offset()
    );
    assert_eq!(
        UsizeConsts::OFFSET_BAR + 96,
        OffsetConsts::OFFSET_BAR.offset()
    );
    assert_eq!(
        UsizeConsts::OFFSET_BAZ + 96,
        OffsetConsts::OFFSET_BAZ.offset()
    );

    assert_eq!(AttributeConsts::OFFSET_FOO, Foo::OFFSET_FOO,);
    assert_eq!(AttributeConsts::OFFSET_BAR, Foo::OFFSET_BAR,);
    assert_eq!(AttributeConsts::OFFSET_BAZ, "nope");
}
