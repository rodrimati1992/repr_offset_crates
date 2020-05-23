#[doc(hidden)]
#[macro_export]
macro_rules! _priv_for_permutations_test {
    (
        alignment =  $p:ident,
        struct $struct:ident;
    ) => {
        $crate::_priv_for_permutations_test!{
            @struct
            alignment =  $p,
            struct $struct;

            type_params(
                (Align16<()>, Align8<()>, Align4<()>, Align2<()>)
                (Align16<()>, Align8<u16>, Align4<u64>, Align2<u128>)
                (Align8<u32>, Align16<()>, Vec<()>, u8)
                ((), Packed8<()>, Packed4<()>, Packed2<()>)
                ((), Packed8<Align16<()>>, Packed4<u64>, Packed2<u128>)
                (Packed1<u128>, Packed2<usize>, Vec<()>, u8)
                (u16, u32, u64, u128)
                ([u64;0], [u8;2], [u8;4], [u8;8])
            )
        }
    };
    (@struct
        alignment =  $p:ident,
        struct $struct:ident;
        type_params(
            $($type_params:tt)*
        )
    )=>{{
        use $crate::types_for_tests::{
            StructReprC,
            StructPacked,StructPacked2,StructPacked4,StructPacked8,
            StructAlign2,StructAlign4,StructAlign8,
            Packed1,Packed2,Packed4,Packed8,Packed16,
            Align2,Align4,Align8,Align16,
        };

        use $crate::_priv_offset_of;

        $(
            $crate::_priv_for_permutations_test!{@type_params $p $struct $type_params }
        )*
    }};
    (@type_params $p:ident $struct:ident ($p0:ty,$p1:ty, $p2:ty, $p3:ty) )=>{
        $crate::_priv_for_permutations_test!{@type_param $p $struct<$p0, $p1, $p2, $p3> }
        $crate::_priv_for_permutations_test!{@type_param $p $struct<$p3, $p2, $p1, $p0> }

        $crate::_priv_for_permutations_test!{@type_param $p $struct<$p3, $p1, $p2, $p0> }
        $crate::_priv_for_permutations_test!{@type_param $p $struct<$p0, $p2, $p1, $p3> }
    };
    (@type_param $p:ident $struct:ident<$param0:ty, $param1:ty, $param2:ty, $param3:ty> )=>{{
        #[inline(never)]
        fn permutation_test() {
            type This = $struct<$param0, $param1, $param2, $param3>;
            type Consts = $struct<(), ($param0, $param1, $param2, $param3), (), ()>;
            //println!("{}", std::any::type_name::<This>());
            let value = This::default();
            assert_eq!( Consts::OFFSET_A.offset() , _priv_offset_of!(value, a, $p) );
            assert_eq!( Consts::OFFSET_B.offset() , _priv_offset_of!(value, b, $p) );
            assert_eq!( Consts::OFFSET_C.offset() , _priv_offset_of!(value, c, $p) );
            assert_eq!( Consts::OFFSET_D.offset() , _priv_offset_of!(value, d, $p) );
        }
        permutation_test();
    }};
}
