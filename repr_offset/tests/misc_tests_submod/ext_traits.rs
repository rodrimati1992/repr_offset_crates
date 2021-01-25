use repr_offset::{
    ext::{ROExtAcc, ROExtOps, ROExtRawAcc, ROExtRawMutAcc, ROExtRawMutOps, ROExtRawOps},
    for_examples::{ReprC, ReprPacked},
    pub_off,
    tstr::TS,
    FieldOffset, GetPubFieldOffset,
};

type SB = TS!(b);
type SD = TS!(d);

fn call_all_ops_methods<S, A>(mut make_both: impl FnMut() -> (S, S))
where
    S: GetPubFieldOffset<SB, Field = usize, Alignment = A>,
    S: GetPubFieldOffset<SD, Field = usize, Alignment = A>,
    S: ROExtAcc + ROExtOps<A>,
    *const S: ROExtRawAcc + ROExtRawOps<A, Target = S>,
    *mut S: ROExtRawMutAcc + ROExtRawMutOps<A, Target = S>,
{
    let off_b: FieldOffset<S, usize, A> = pub_off!(b);
    let off_d: FieldOffset<S, usize, A> = pub_off!(d);

    let init = |(mut left, mut right): (S, S)| {
        left.f_replace(off_b, 13);
        left.f_replace(off_d, 21);
        right.f_replace(off_b, 34);
        right.f_replace(off_d, 55);
        (left, right)
    };

    {
        let (mut left, _) = init(make_both());

        unsafe {
            left.f_get_mut_ptr(off_b).write_unaligned(103);
            left.f_get_mut_ptr(off_d).write_unaligned(105);

            assert_eq!(left.f_get_ptr(off_b).read_unaligned(), 103);
            assert_eq!(left.f_get_ptr(off_d).read_unaligned(), 105);
        }
        unsafe {
            let left_ptr: *mut _ = &mut left;
            left_ptr.f_raw_get_mut(off_b).write_unaligned(55);
            left_ptr.f_raw_get_mut(off_d).write_unaligned(89);
        }
        unsafe {
            let left_ptr: *const _ = &left;
            assert_eq!(left_ptr.f_raw_get(off_b).read_unaligned(), 55);
            assert_eq!(left_ptr.f_raw_get(off_d).read_unaligned(), 89);
        }
    }
    {
        let (mut left, _) = make_both();
        unsafe {
            let left_ptr: *mut _ = &mut left;
            left_ptr.f_write(off_b, 5);
            left_ptr.f_write(off_d, 8);

            let left_ptr: *const _ = &left;
            assert_eq!(left_ptr.f_read(off_b), 5);
            assert_eq!(left_ptr.f_read(off_d), 8);

            assert_eq!(left_ptr.f_read_copy(off_b), 5);
            assert_eq!(left_ptr.f_read_copy(off_d), 8);
        }
        assert_eq!(left.f_get_copy(off_b), 5);
        assert_eq!(left.f_get_copy(off_d), 8);

        assert_eq!(left.f_replace(off_b, 13), 5);
        assert_eq!(left.f_replace(off_d, 21), 8);

        assert_eq!(left.f_get_copy(off_b), 13);
        assert_eq!(left.f_get_copy(off_d), 21);

        unsafe {
            let left_ptr: *mut _ = &mut left;
            assert_eq!(left_ptr.f_replace_raw(off_b, 34), 13);
            assert_eq!(left_ptr.f_replace_raw(off_d, 55), 21);
        }
    }

    {
        let (mut left, mut right) = init(make_both());

        left.f_swap(off_b, &mut right);
        left.f_swap(off_d, &mut right);

        assert_eq!(left.f_get_copy(off_b), 34);
        assert_eq!(left.f_get_copy(off_d), 55);
        assert_eq!(right.f_get_copy(off_b), 13);
        assert_eq!(right.f_get_copy(off_d), 21);
    }

    unsafe {
        let (mut left, mut right) = init(make_both());

        let left_ptr: *mut _ = &mut left;
        left_ptr.f_copy_from(off_b, &mut right);
        left_ptr.f_copy_from(off_d, &mut right);

        assert_eq!(left.f_get_copy(off_b), 34);
        assert_eq!(left.f_get_copy(off_d), 55);
        assert_eq!(right.f_get_copy(off_b), 34);
        assert_eq!(right.f_get_copy(off_d), 55);
    }

    unsafe {
        let (mut left, mut right) = init(make_both());

        let left_ptr: *mut _ = &mut left;
        left_ptr.f_copy_from_nonoverlapping(off_b, &mut right);
        left_ptr.f_copy_from_nonoverlapping(off_d, &mut right);

        assert_eq!(left.f_get_copy(off_b), 34);
        assert_eq!(left.f_get_copy(off_d), 55);
        assert_eq!(right.f_get_copy(off_b), 34);
        assert_eq!(right.f_get_copy(off_d), 55);
    }

    unsafe {
        let (mut left, mut right) = init(make_both());

        let left_ptr: *mut _ = &mut left;
        left_ptr.f_swap_raw(off_b, &mut right);
        left_ptr.f_swap_raw(off_d, &mut right);

        assert_eq!(left.f_get_copy(off_b), 34);
        assert_eq!(left.f_get_copy(off_d), 55);
        assert_eq!(right.f_get_copy(off_b), 13);
        assert_eq!(right.f_get_copy(off_d), 21);
    }

    unsafe {
        let (mut left, mut right) = init(make_both());

        let left_ptr: *mut _ = &mut left;
        left_ptr.f_swap_nonoverlapping(off_b, &mut right);
        left_ptr.f_swap_nonoverlapping(off_d, &mut right);

        assert_eq!(left.f_get_copy(off_b), 34);
        assert_eq!(left.f_get_copy(off_d), 55);
        assert_eq!(right.f_get_copy(off_b), 13);
        assert_eq!(right.f_get_copy(off_d), 21);
    }
}

#[test]
fn test_all_ext_ops_traits() {
    call_all_ops_methods(|| {
        (
            ReprPacked {
                a: 101u8,
                b: 102usize,
                c: 103u8,
                d: 104usize,
            },
            ReprPacked {
                a: 201u8,
                b: 202usize,
                c: 203u8,
                d: 204usize,
            },
        )
    });

    call_all_ops_methods(|| {
        (
            ReprC {
                a: 101u8,
                b: 102usize,
                c: 103u8,
                d: 104usize,
            },
            ReprC {
                a: 201u8,
                b: 202usize,
                c: 203u8,
                d: 204usize,
            },
        )
    });
}
