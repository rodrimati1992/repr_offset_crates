/*
fn main() {
    fn as_ty(b: bool) -> &'static str {
        if b {
            "$t"
        } else {
            "$f"
        }
    }

    for elem_count in 0..=4 {
        for bits in 0..1 << elem_count {
            let is_optional = (0..elem_count)
                .map(|i| (bits >> i) & 1 != 0)
                .collect::<Vec<bool>>();

            let tup = is_optional.iter().copied().map(as_ty).collect::<Vec<_>>();
            let any_optional = is_optional.iter().cloned().any(|x| !x);

            println!(
                "({tup})={output},",
                tup = tup.join(","),
                output = if any_optional { "$f" } else {"Carry" },
            )
        }
    }
}

*/

macro_rules! impl_all_trait_for_tuples {
    (
        macro = $the_macro:ident,
        true = $t:ident,
        false = $f:ident,
    ) => {
        $the_macro! {small=> ()=Carry }
        $the_macro! {small=> ($f,)=$f }
        $the_macro! {small=> ($t,)=Carry }
        $the_macro! {small=> ($f,$f)=$f }
        $the_macro! {small=> ($t,$f)=$f }
        $the_macro! {small=> ($f,$t)=$f }
        $the_macro! {small=> ($t,$t)=Carry }
        $the_macro! {small=> ($f,$f,$f)=$f }
        $the_macro! {small=> ($t,$f,$f)=$f }
        $the_macro! {small=> ($f,$t,$f)=$f }
        $the_macro! {small=> ($t,$t,$f)=$f }
        $the_macro! {small=> ($f,$f,$t)=$f }
        $the_macro! {small=> ($t,$f,$t)=$f }
        $the_macro! {small=> ($f,$t,$t)=$f }
        $the_macro! {small=> ($t,$t,$t)=Carry }
        $the_macro! {small=> ($f,$f,$f,$f)=$f }
        $the_macro! {small=> ($t,$f,$f,$f)=$f }
        $the_macro! {small=> ($f,$t,$f,$f)=$f }
        $the_macro! {small=> ($t,$t,$f,$f)=$f }
        $the_macro! {small=> ($f,$f,$t,$f)=$f }
        $the_macro! {small=> ($t,$f,$t,$f)=$f }
        $the_macro! {small=> ($f,$t,$t,$f)=$f }
        $the_macro! {small=> ($t,$t,$t,$f)=$f }
        $the_macro! {small=> ($f,$f,$f,$t)=$f }
        $the_macro! {small=> ($t,$f,$f,$t)=$f }
        $the_macro! {small=> ($f,$t,$f,$t)=$f }
        $the_macro! {small=> ($t,$t,$f,$t)=$f }
        $the_macro! {small=> ($f,$f,$t,$t)=$f }
        $the_macro! {small=> ($t,$f,$t,$t)=$f }
        $the_macro! {small=> ($f,$t,$t,$t)=$f }
        $the_macro! {small=> ($t,$t,$t,$t)=Carry }

        $the_macro! {large=>(A0,A1,A2,A3,),A4,}
        $the_macro! {large=>(A0,A1,A2,A3,),A4,A5,}
        $the_macro! {large=>(A0,A1,A2,A3,),A4,A5,A6,}
        $the_macro! {large=>(A0,A1,A2,A3,),(A4,A5,A6,A7,),}
        $the_macro! {large=>(A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,}
        $the_macro! {large=>(A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,A9,}
        $the_macro! {large=>(A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,A9,A10,}
        $the_macro! {large=>(A0,A1,A2,A3,),(A4,A5,A6,A7,),(A8,A9,A10,A11,),}
    };
}
