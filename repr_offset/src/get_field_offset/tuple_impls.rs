use crate::{
    alignment::{Aligned, Alignment, CombineAlignment},
    get_field_offset::{
        FieldOffsetWithVis, GetFieldOffset, ImplGetNestedFieldOffset, ImplsGetFieldOffset,
    },
    privacy::{CombinePrivacy, IsPublic, Privacy},
};

macro_rules! tuple_impl {
    (
        [$($field:ident)*],
        [$($tp:ident)*],
        [$($tp_trail:ident)*],
        $first:ident,
        $last:ident
    ) => {
        unsafe impl<T, $($field,)*>
            GetFieldOffset<($($field,)*)>
        for T
        where
            T: ImplsGetFieldOffset,
            ImplGetNestedFieldOffset<T>: GetFieldOffset<($($field,)*)>
        {
            type Type = <ImplGetNestedFieldOffset<T> as GetFieldOffset<($($field,)*)>>::Type;
            type Alignment = <ImplGetNestedFieldOffset<T> as GetFieldOffset<($($field,)*)>>::Alignment;
            type Privacy = <ImplGetNestedFieldOffset<T> as GetFieldOffset<($($field,)*)>>::Privacy;

            const OFFSET_WITH_VIS: FieldOffsetWithVis<
                Self,
                Self::Privacy,
                ($($field,)*),
                Self::Type,
                Self::Alignment,
            > = unsafe{
                <ImplGetNestedFieldOffset<T> as GetFieldOffset<($($field,)*)>>::OFFSET_WITH_VIS
                    .cast_struct()
            };
        }

        unsafe impl<$($tp,)* $($field,)* $last, CombAlign, CombPriv>
            GetFieldOffset<($($field,)*)>
        for ImplGetNestedFieldOffset<$first>
        where
            $first: ImplsGetFieldOffset,
            $(
                $tp: GetFieldOffset<$field, Type = $tp_trail>,
            )*
            ($($tp::Alignment,)*): CombineAlignment<Aligned, Output = CombAlign>,
            ($($tp::Privacy,)*): CombinePrivacy<IsPublic, Output = CombPriv>,
            CombAlign: Alignment,
            CombPriv: Privacy,
        {
            type Type = $last;
            type Alignment = CombAlign;
            type Privacy = CombPriv;

            const OFFSET_WITH_VIS: FieldOffsetWithVis<
                Self,
                Self::Privacy,
                ($($field,)*),
                $last,
                Self::Alignment,
            > = unsafe{
                let offset = {
                    0
                    $(
                        + <$tp as GetFieldOffset<$field>>::OFFSET_WITH_VIS
                            .private_field_offset()
                            .offset()
                    )*
                };

                FieldOffsetWithVis::new(offset)
            };
        }
    };
}

/*
fn main(){
    for len in 2..=8 {
        print!("tuple_impl! {{\n\t[");
        for i in 0..len {
            print!("F{} ", i)
        }
        print!("],\n\t[");
        for i in 0..len {
            print!("L{} ", i)
        }
        print!("],\n\t[");
        for i in 1..=len {
            print!("L{} ", i);
        }
        println!("],\n\tL{}, L{}\n}}", 0, len);
    }
}

*/

tuple_impl! {
    [F0 F1 ],
    [L0 L1 ],
    [L1 L2 ],
    L0, L2
}
tuple_impl! {
    [F0 F1 F2 ],
    [L0 L1 L2 ],
    [L1 L2 L3 ],
    L0, L3
}
tuple_impl! {
    [F0 F1 F2 F3 ],
    [L0 L1 L2 L3 ],
    [L1 L2 L3 L4 ],
    L0, L4
}
tuple_impl! {
    [F0 F1 F2 F3 F4 ],
    [L0 L1 L2 L3 L4 ],
    [L1 L2 L3 L4 L5 ],
    L0, L5
}
tuple_impl! {
    [F0 F1 F2 F3 F4 F5 ],
    [L0 L1 L2 L3 L4 L5 ],
    [L1 L2 L3 L4 L5 L6 ],
    L0, L6
}
tuple_impl! {
    [F0 F1 F2 F3 F4 F5 F6 ],
    [L0 L1 L2 L3 L4 L5 L6 ],
    [L1 L2 L3 L4 L5 L6 L7 ],
    L0, L7
}
tuple_impl! {
    [F0 F1 F2 F3 F4 F5 F6 F7 ],
    [L0 L1 L2 L3 L4 L5 L6 L7 ],
    [L1 L2 L3 L4 L5 L6 L7 L8 ],
    L0, L8
}
