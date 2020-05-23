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
            |$variable:ident, $off0:ident, $off1:ident, $off2:ident, $off3:ident|
            $using:expr
        ),* $(,)?
    )=>{
        $(
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                $variable,
                $using,
                ($e0, $off0)
                ($e1, $off1)
                ($e2, $off2)
                ($e3, $off3)
            }
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                $variable,
                $using,
                ($e3, $off3)
                ($e2, $off2)
                ($e1, $off1)
                ($e0, $off0)
            }
            $crate::_priv_run_with_types!{
                @inner
                type_constructors $type_constructors,
                $variable,
                $using,
                ($e2, $off2)
                ($e3, $off3)
                ($e0, $off0)
                ($e1, $off1)
            }
        )*
    };
    (@inner
        type_constructors[ $($types:ident),* $(,)? ],
        $variable:ident,
        $using:expr,
        ($a:expr, $a_off:ident)
        ($b:expr, $b_off:ident)
        ($c:expr, $c_off:ident)
        ($d:expr, $d_off:ident)
    )=>{
        $({
            fn function(){
                {
                    let mut $variable = $types{a:$a, b:$b, c:$c, d:$d };
                    let $a_off = $types::OFFSET_A;
                    let $b_off = $types::OFFSET_B;
                    let $c_off = $types::OFFSET_C;
                    let $d_off = $types::OFFSET_D;
                    $using
                }
            }
            function();
        })*
    };
}
