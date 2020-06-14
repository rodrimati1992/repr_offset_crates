#[doc(hidden)]
#[macro_use]
mod for_permutations;

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_offset_of {
    ( $instance:ident, $field:tt, $alignment:ident ) => {
        unsafe {
            let field_ptr = $crate::_priv_address_of!($alignment, $instance.$field);
            let struct_ptr = $crate::_priv_address_of!(aligned, $instance);
            field_ptr - struct_ptr
        }
    };
}

#[cfg(not(feature = "priv_raw_ref"))]
#[doc(hidden)]
#[macro_export]
macro_rules! _priv_address_of {
    ( aligned, $($expr:tt)* ) => ( & ($($expr)*) as *const _ as usize );
}

#[cfg(feature = "priv_raw_ref")]
#[doc(hidden)]
#[macro_export]
macro_rules! _priv_address_of {
    ( aligned, $($expr:tt)* ) => ( & ($($expr)*) as *const _ as usize );
    ( packed, $($expr:tt)* ) => ( &raw const ($($expr)*) as usize );
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_run_with_types {
    (
        type_constructors $type_constructors:tt,
        $(
            ($e0:expr, $e1:expr, $e2:expr, $e3:expr $(,)?)
            ($f0:expr, $f1:expr, $f2:expr, $f3:expr $(,)?)
            |
                $variable:ident , $other:ident,
                $off0:ident, $off1:ident, $off2:ident, $off3:ident
            |
            $($using:block)+
        ),* $(,)?
    )=>{
        $(
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                ($($using)*)
                (
                    $variable,
                    $other,
                    ($e0, $f0, $off0)
                    ($e1, $f1, $off1)
                    ($e2, $f2, $off2)
                    ($e3, $f3, $off3)
                )
            }
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                ($($using)*)
                (
                    $variable,
                    $other,
                    ($e3, $f3, $off3)
                    ($e2, $f2, $off2)
                    ($e1, $f1, $off1)
                    ($e0, $f0, $off0)
                )
            }
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                ($($using)*)
                (
                    $variable,
                    $other,
                    ($e2, $f2, $off2)
                    ($e3, $f3, $off3)
                    ($e0, $f0, $off0)
                    ($e1, $f1, $off1)
                )
            }
        )*
    };
    (@inner
        type_constructors[ $($types:ident),* $(,)? ],
        $using_blocks:tt
        $shared:tt
    )=>{
        $({
            $crate::_priv_run_with_types!{
                @inner-1
                type_constructor = $types,
                $using_blocks
                $shared
            }
        })*
    };
    (@inner-1
        type_constructor = $type:ident,
        ($($using:block)*)
        (
            $variable:ident,
            $other:ident,
            ($ea:expr, $fa:expr, $a_off:ident)
            ($eb:expr, $fb:expr, $b_off:ident)
            ($ec:expr, $fc:expr, $c_off:ident)
            ($ed:expr, $fd:expr, $d_off:ident)
        )
    )=>{{
        fn function(){
            let make_variable = || $type{a:$ea, b:$eb, c:$ec, d:$ed };
            let make_other = || $type{a:$fa, b:$fb, c:$fc, d:$fd };
            let $a_off = $type::OFFSET_A;
            let $b_off = $type::OFFSET_B;
            let $c_off = $type::OFFSET_C;
            let $d_off = $type::OFFSET_D;
            $({
                let mut $variable = make_variable();
                let mut $other = make_other();
                $using
            })*
        }
        function();
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_swap_tests {
    (
        $offset:expr,
        get_with = $get_with:expr,
        variables($var0:ident, $var1:ident)
        values($val0:expr, $val1:expr)
    ) => {{
        assert_eq!($get_with($offset, &$var0), $val0);
        assert_eq!($get_with($offset, &$var1), $val1);

        // Testing the identity swap
        {
            let ptr: *mut _ = &mut $var0;
            $offset.swap(ptr, ptr);
            assert_eq!($get_with($offset, &$var0), $val0);
        }

        $offset.swap(&mut $var0, &mut $var1);
        assert_eq!($get_with($offset, &$var0), $val1);
        assert_eq!($get_with($offset, &$var1), $val0);

        $offset.swap_nonoverlapping(&mut $var0, &mut $var1);
        assert_eq!($get_with($offset, &$var0), $val0);
        assert_eq!($get_with($offset, &$var1), $val1);

        $offset.swap_mut(&mut $var0, &mut $var1);
        assert_eq!($get_with($offset, &$var0), $val1);
        assert_eq!($get_with($offset, &$var1), $val0);

        $offset.swap_mut(&mut $var0, &mut $var1);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_copy_tests {
    (
        $offset:expr,
        get_with = $get_with:expr,
        variables($var0:ident, $var1:ident)
        values($($val:expr),* $(,)?)
    ) => {{
        $({
            assert_ne!( $get_with($offset,&$var0), $val );
            assert_ne!( $get_with($offset,&$var1), $val );
            $offset.write(&mut $var0, $val);
            {
                let ptr: *mut _ = &mut $var0;
                $offset.copy(ptr, ptr);
            }
            $offset.copy(&$var0, &mut $var1);
            assert_eq!( $get_with($offset,&$var0), $val );
            assert_eq!( $get_with($offset,&$var1), $val );
        })*
        $({
            assert_ne!( $get_with($offset,&$var0), $val );
            assert_ne!( $get_with($offset,&$var1), $val );
            $offset.write(&mut $var0, $val);
            $offset.copy_nonoverlapping(&$var0, &mut $var1);
            assert_eq!( $get_with($offset,&$var0), $val );
            assert_eq!( $get_with($offset,&$var1), $val );
        })*
    }};

}
