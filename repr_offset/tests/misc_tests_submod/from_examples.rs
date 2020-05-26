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
