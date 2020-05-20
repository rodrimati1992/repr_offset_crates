#[doc(hidden)]
#[macro_use]
mod for_permutations;

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_offset_of {
    ( $instance:ident, $field:tt, $packing:ident ) => {
        unsafe {
            let field_ptr = $crate::_priv_address_of!($packing, $instance.$field);
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
