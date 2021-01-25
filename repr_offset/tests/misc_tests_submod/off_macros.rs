use repr_offset::{
    for_examples::ReprC, off, pub_off, Aligned, FieldOffset, ROExtAcc, OFF, PUB_OFF,
};

#[derive(Debug, PartialEq)]
struct MoveOnly(u32);

type RFooInner = ReprC<i16, i32, i64, i128>;
type RFoo = ReprC<u8, RFooInner, MoveOnly, Option<u32>>;

const RFOO: RFoo = ReprC {
    a: 5,
    b: ReprC {
        a: 203,
        b: 205,
        c: 208,
        d: 213,
    },
    c: MoveOnly(221),
    d: Some(13),
};

const _CONST_EXECUTABLE: () = {
    let _: FieldOffset<_, u8, Aligned> = OFF!(RFoo, a);
    let _: FieldOffset<_, u8, Aligned> = PUB_OFF!(RFoo, a);

    let _: FieldOffset<_, i32, Aligned> = OFF!(RFoo, b.b);
    let _: FieldOffset<_, i32, Aligned> = PUB_OFF!(RFoo, b.b);

    let _: FieldOffset<_, u8, Aligned> = OFF!(RFoo, a);
    let _: FieldOffset<_, u8, Aligned> = PUB_OFF!(RFoo, a);

    let _: FieldOffset<_, i32, Aligned> = OFF!(RFoo, b.b);
    let _: FieldOffset<_, i32, Aligned> = PUB_OFF!(RFoo, b.b);

    let _: FieldOffset<ReprC<_, u8, u8, u8>, u8, Aligned> = OFF!(ReprC, a);
    let _: FieldOffset<ReprC<u8, _, u8, u8>, u16, Aligned> = PUB_OFF!(ReprC, b);

    let _: FieldOffset<ReprC<_, u8, u8, u8>, u8, Aligned> = OFF!(ReprC, a);
    let _: FieldOffset<ReprC<u8, _, u8, u8>, u16, Aligned> = PUB_OFF!(ReprC, b);

    let _: FieldOffset<_, (), Aligned> = OFF!(ReprC, a);
    let _: FieldOffset<_, (), Aligned> = PUB_OFF!(ReprC, a);

    let _: FieldOffset<_, (), Aligned> = OFF!(ReprC, b);
    let _: FieldOffset<_, (), Aligned> = PUB_OFF!(ReprC, b);
};

#[test]
fn capitalized_off_macro() {
    {
        let foo = RFOO;

        assert_eq!(foo.f_get(pub_off!(b.a)), &203);
        assert_eq!(foo.f_get(pub_off!(*&foo, b.b)), &205);
        assert_eq!(foo.f_get(pub_off!(b.c)), &208);
        assert_eq!(foo.f_get(pub_off!(b.d)), &213);
        assert_eq!(foo.f_get(pub_off!(*&foo, c)), &MoveOnly(221));
    }
    {
        let foo = RFOO;

        assert_eq!(foo.f_get(off!(*&foo, a)), &5);
        assert_eq!(foo.f_get(off!(b.c)), &208);
        assert_eq!(foo.f_get(off!(b.d)), &213);
        assert_eq!(foo.f_get(off!(c)), &MoveOnly(221));
        assert_eq!(foo.f_get(off!(*&foo, d)), &Some(13));
    }
    {
        let foo = RFOO;

        assert_eq!(foo.f_get(OFF!(RFoo, a)), &5);
        assert_eq!(foo.f_get(OFF!(RFoo, b.c)), &208);
        assert_eq!(foo.f_get(OFF!(RFoo, b.d)), &213);
        assert_eq!(foo.f_get(OFF!(RFoo, c)), &MoveOnly(221));
        assert_eq!(foo.f_get(OFF!(ReprC, d)), &Some(13));
    }
    {
        let foo = RFOO;

        assert_eq!(foo.f_get(PUB_OFF!(RFoo, a)), &5);
        assert_eq!(foo.f_get(PUB_OFF!(RFoo, b.c)), &208);
        assert_eq!(foo.f_get(PUB_OFF!(ReprC, b.d)), &213);
        assert_eq!(foo.f_get(PUB_OFF!(RFoo, c)), &MoveOnly(221));
        assert_eq!(foo.f_get(PUB_OFF!(ReprC, d)), &Some(13));
    }
}
