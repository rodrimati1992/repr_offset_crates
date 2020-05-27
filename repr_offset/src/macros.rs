/// Declares a sequence of associated constants with the offsets of the listed fields.
///
/// # Safety
///
/// Callers must ensure that:
///
/// - The type that the offsets are for is a `#[repr(C)]` struct.
///
/// - All field types are listed,in declaration order.
///
/// - The `alignment` parameter is [`Unaligned`] if the struct is `#[repr(C,packed)]`,
/// and [`Aligned`] if it's not.
///
/// # Parameters
///
/// ### `Self`
///
/// The optional `Self` parameter overrides which struct the [`FieldOffset`] constants
/// (that this outputs) are an offset inside of.
///
/// ### `alignment`
///
/// The `alignment` parameter can be either [`Aligned`] or [`Unaligned`],
/// and describes whether the fields are aligned or potentially unaligned,
/// changing how fields are accessed in [`FieldOffset`] methods.
///
/// ### `usize_offsets`
///
/// The optional `usize_offsets` parameter determines whether type of the
/// generated constants is [`FieldOffset`] or `usize`.<br>
///
/// The valid values for this parameter are:
/// - (not passing this parameter): The constants are [`FieldOffset`]s.
/// - `false`: The constants are [`FieldOffset`]s.
/// - `true`: The constants are `usize`s.
///
///
/// [`Aligned`]: ./struct.Aligned.html
/// [`Unaligned`]: ./struct.Unaligned.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
///
/// # Examples
///
/// ### Syntax example
///
/// This demonstrates the macro being used with all of the syntax.
///
/// ```rust
/// use repr_offset::{unsafe_struct_field_offsets, Aligned};
///
/// #[repr(C)]
/// struct Bar<T: Copy, U>(T,U)
/// where U: Clone;
///
/// unsafe_struct_field_offsets!{
///     // Optional parameter.
///     // Generic parameters from the impl block can be used here.
///     Self = Bar<T, U>,
///
///     alignment =  Aligned,
///
///     // Optional parameter.
///     usize_offsets = false,
///
///     impl[T: Copy, U] Bar<T, U>
///     where[ U: Clone ]
///     {
///         pub const OFFSET_0: T;
///         pub const OFFSET_1: U;
///     }
/// }
///
/// ```
///
/// ### Unaligned struct example
///
/// This demonstrates how you can get a pointer to a field from a pointer to
/// a packed struct (it's UB to use references to fields here).
///
/// ```rust
/// use repr_offset::{unsafe_struct_field_offsets, Unaligned};
///
/// let mut bar = Bar{ mugs: 3, bottles: 5, table: "wooden".to_string() };
///
/// assert_eq!( replace_table(&mut bar, "metallic".to_string()), "wooden".to_string());
/// assert_eq!( replace_table(&mut bar, "granite".to_string()), "metallic".to_string());
/// assert_eq!( replace_table(&mut bar, "carbonite".to_string()), "granite".to_string());
///
/// fn replace_table(this: &mut Bar, replacement: String)-> String{
///     let ptr = Bar::OFFSET_TABLE.get_mut_ptr(this);
///     unsafe{
///         let taken = ptr.read_unaligned();
///         ptr.write_unaligned(replacement);
///         taken
///     }
/// }
///
///
/// #[repr(C,packed)]
/// struct Bar{
///     mugs: u32,
///     bottles: u16,
///     table: String,
/// }
///
/// unsafe_struct_field_offsets!{
///     alignment =  Unaligned,
///
///     impl[] Bar {
///         pub const OFFSET_MUGS: u32;
///         pub const OFFSET_BOTTLES: u16;
///         pub const OFFSET_TABLE: String;
///     }
/// }
///
/// ```
///
///
#[macro_export]
macro_rules! unsafe_struct_field_offsets{
    (
        $( Self = $Self:ty, )?
        alignment =  $alignment:ty,
        $( usize_offsets = $usize_offsets:ident,)?

        $(#[$impl_attr:meta])*
        impl[ $($impl_params:tt)* ] $self:ty
        $(where [ $($where:tt)* ])?
        {
            $(
                $(#[$const_attr:meta])*
                $vis:vis const $offset:ident : $field_ty:ty;
            )*
        }
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_params)*>  $self
        $(where $($where)*)?
        {
            $crate::_priv_usfoi!{
                @setup
                params(
                    Self( $($Self,)? Self, )
                    alignment =  $alignment,
                    usize_offsets($($usize_offsets,)? false,)
                )
                previous(
                    (
                        $crate::_priv_usfoi!(
                            @initial
                            $($usize_offsets)?, 0,
                        ),
                        ()
                    ),
                    $((Self::$offset, $field_ty),)*
                )
                offsets($(
                    $(#[$const_attr])*
                    $vis $offset: $field_ty;
                )*)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_usfoi{
    (@setup
        params $params:tt
        previous( $($prev:tt)* )
        offsets( $($offsets:tt)* )
    )=>{
        $crate::_priv_usfoi!{
            params $params
            params $params
            previous( $($prev)* )
            offsets( $($offsets)* )
        }
    };
    (@initial true, $value:expr, )=>{
        $value
    };
    (@initial $(false)?, $value:expr, )=>{
        $crate::FieldOffset::<_,(),$crate::Aligned>::new($value)
    };
    (@ty true, $Self:ty, $next_ty:ty, $alignment:ty )=>{
        usize
    };
    (@ty false, $Self:ty, $next_ty:ty, $alignment:ty )=>{
        $crate::FieldOffset<$Self,$next_ty,$alignment>
    };
    (@val true, $Self:ty, $prev:expr, $prev_ty:ty, $next_ty:ty )=>{
        $crate::offset_calc::next_field_offset::<$Self, $prev_ty, $next_ty>( $prev )
    };
    (@val false, $Self:ty, $prev:expr, $prev_ty:ty, $next_ty:ty )=>{
        $prev.next_field_offset()
    };
    (
        params $params:tt
        params(
            Self( $Self:ty, $($_ignored_Self:ty,)? )
            alignment =  $alignment:ty,
            usize_offsets($usize_offsets:ident, $($_ignored_io:ident,)? )
        )
        previous( ($prev_offset:expr, $prev_ty:ty), $($prev:tt)* )
        offsets(
            $(#[$const_attr:meta])*
            $vis:vis $offset:ident : $field_ty:ty;
            $($next:tt)*
        )
    )=>{
        $(#[$const_attr])*
        $vis const $offset:
            $crate::_priv_usfoi!(
                @ty $usize_offsets, $Self, $field_ty, $alignment
            )
        = unsafe{
            $crate::_priv_usfoi!(
                @val
                $usize_offsets, $Self, $prev_offset, $prev_ty, $field_ty
            )
        };

        $crate::_priv_usfoi!{
            params $params
            params $params
            previous($($prev)*)
            offsets($($next)*)
        }
    };
    (
        params $params:tt
        params $params2:tt
        previous($($prev:tt)*)
        offsets()
    )=>{};
}
