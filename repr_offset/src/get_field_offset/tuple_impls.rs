use crate::{
    get_field_offset::PrivateFieldOffset, Aligned, Alignment, CombinePacking, FieldOffset,
    GetFieldOffset,
};

macro_rules! tuple_impl {
    (
        [$($field:ident)*],
        [$($tp:ident)*],
        [$($tp_trail:ident)*],
        $first:ident,
        $last:ident
    ) => {
        unsafe impl<$($tp,)* $($field,)* $last, CombAlign> GetFieldOffset<($($field,)*)> for $first
        where
            $(
                $tp: GetFieldOffset<$field, Field = $tp_trail>,
            )*
            ($($tp::Alignment,)*): CombinePacking<Aligned, Output = CombAlign>,
            CombAlign: Alignment,
        {
            type This = $first::This;
            type Field = $last;
            type Alignment = CombAlign;

            const PRIV_OFFSET: PrivateFieldOffset<
                Self,
                ($($field,)*),
                Self::This,
                $last,
                Self::Alignment,
            > = unsafe{
                let offset = {
                    0
                    $( + <$tp as GetFieldOffset<$field>>::PRIV_OFFSET.private_field_offset().offset() )*
                };

                PrivateFieldOffset::new(FieldOffset::new(offset))
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
