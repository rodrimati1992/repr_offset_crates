use repr_offset::types_for_tests::StructPacked;

use std::mem::ManuallyDrop;

#[test]
fn replace_struct_field() {
    use repr_offset::{unsafe_struct_field_offsets, Unaligned};

    let mut bar = Bar {
        mugs: 3,
        bottles: 5,
        table: "wooden".to_string(),
    };

    assert_eq!(
        replace_table(&mut bar, "metallic".to_string()),
        "wooden".to_string()
    );
    assert_eq!(
        replace_table(&mut bar, "granite".to_string()),
        "metallic".to_string()
    );
    assert_eq!(
        replace_table(&mut bar, "carbonite".to_string()),
        "granite".to_string()
    );

    fn replace_table(this: &mut Bar, replacement: String) -> String {
        let ptr = Bar::OFFSET_TABLE.get_mut_ptr(this);
        unsafe {
            let taken = ptr.read_unaligned();
            ptr.write_unaligned(replacement);
            taken
        }
    }

    #[repr(C, packed)]
    struct Bar {
        mugs: u32,
        bottles: u16,
        table: String,
    }

    unsafe_struct_field_offsets! {
        alignment =  Unaligned,

        impl[] Bar {
            pub const OFFSET_MUGS: u32;
            pub const OFFSET_BOTTLES: u16;
            pub const OFFSET_TABLE: String;
        }
    }
}

#[test]
fn reading_out_unaligned() {
    let mut this = ManuallyDrop::new(StructPacked {
        a: 5,
        b: 8,
        c: "oh,hi".to_string(),
        d: (),
    });
    let ptr = &mut *this;
    unsafe {
        assert_eq!(StructPacked::OFFSET_A.read(ptr), 5);
        assert_eq!(StructPacked::OFFSET_B.read(ptr), 8);
        assert_eq!(StructPacked::OFFSET_C.read(ptr), "oh,hi".to_string());
    }
}

#[test]
fn accessing_nested_field_in_packed() {
    use repr_offset::{Aligned, FieldOffset, Unaligned};
    use repr_offset_derive::ReprOffset;

    #[repr(C, packed)]
    #[derive(ReprOffset)]
    struct Pack {
        x: u8,
        y: NestedC,
    }

    #[repr(C)]
    #[derive(ReprOffset)]
    struct NestedC {
        name: &'static str,
        years: usize,
    }

    let this = Pack {
        x: 0,
        y: NestedC {
            name: "John",
            years: 13,
        },
    };

    const OFFY: FieldOffset<Pack, NestedC, Unaligned> = Pack::OFFSET_Y;

    let _: FieldOffset<NestedC, &'static str, Aligned> = NestedC::OFFSET_NAME;
    let _: FieldOffset<NestedC, usize, Aligned> = NestedC::OFFSET_YEARS;

    // As you can see `FieldOffset::add` combines two offsets,
    // allowing you to access a nested field with a single `FieldOffset`.
    const OFF_NAME: FieldOffset<Pack, &'static str, Unaligned> = OFFY.add(NestedC::OFFSET_NAME);
    const OFF_YEARS: FieldOffset<Pack, usize, Unaligned> = OFFY.add(NestedC::OFFSET_YEARS);

    assert_eq!(OFF_NAME.get_copy(&this), "John");
    assert_eq!(OFF_YEARS.get_copy(&this), 13);

    unsafe {
        let x_ptr = OFFY.get_ptr(&this);

        // This code is undefined behavior,
        // because the `NestedC`'s offsets require the passed in pointer to be aligned.
        //
        // The `FieldOffset::add` method handles that just fine,
        // because it combines the Alignment type parameter of both `FieldOffset`s.
        //
        // assert_eq!(NestedC::OFFSET_NAME.read(x_ptr), "John" );
        // assert_eq!(NestedC::OFFSET_YEARS.read(x_ptr), 13 );

        assert_eq!(NestedC::OFFSET_NAME.to_unaligned().read(x_ptr), "John");
        assert_eq!(NestedC::OFFSET_YEARS.to_unaligned().read(x_ptr), 13);
    }
}

#[test]
fn to_unaligned_example() {
    use repr_offset::for_examples::{ReprC, ReprPacked};

    type Inner = ReprC<usize, &'static str>;
    type Outer = ReprPacked<u8, Inner>;
    let inner = ReprC {
        a: 3,
        b: "5",
        c: (),
        d: (),
    };
    let outer: Outer = ReprPacked {
        a: 21,
        b: inner,
        c: (),
        d: (),
    };
    let inner_ptr: *const Inner = Outer::OFFSET_B.get_ptr(&outer);
    unsafe {
        assert_eq!(Inner::OFFSET_A.to_unaligned().read_copy(inner_ptr), 3);
        assert_eq!(Inner::OFFSET_B.to_unaligned().read_copy(inner_ptr), "5");
    }
}

#[test]
fn to_aligned_example() {
    use repr_offset::for_examples::ReprPacked2;
    use repr_offset::{FieldOffset, Unaligned};

    type This = ReprPacked2<u8, u16, (), ()>;

    let _: FieldOffset<This, u8, Unaligned> = This::OFFSET_A;
    let _: FieldOffset<This, u16, Unaligned> = This::OFFSET_B;
    let this: This = ReprPacked2 {
        a: 89,
        b: 144,
        c: (),
        d: (),
    };
    unsafe {
        assert_eq!(This::OFFSET_A.to_aligned().get(&this), &89);
        assert_eq!(This::OFFSET_B.to_aligned().get(&this), &144);
    }
}
