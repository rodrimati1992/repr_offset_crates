/// Declares a sequence of associated constants with the offsets of the listed fields.
///
/// # Safety
///
/// Callers must ensure that:
///
/// - All field types are listed,in declaration order.
///
/// - The `packing` parameter is [`Packed`] if the struct is `#[repr(C,packed)]`,
/// and [`Aligned`] if it's not.
///
/// # Parameters
///
/// The optional `Self` parameter overrides which struct the [`FieldOffset`] constants
/// (that this outputs) are an offset inside of.
///
/// The `packing` parameter can be either [`Aligned`] or [`Packed`],
/// and describes whether the fields are aligned or potentially unaligned,
/// changing how fields are accessed in [`FieldOffset`] methods.
///
///
/// [`Aligned`]: ./struct.Aligned.html
/// [`Packed`]: ./struct.Packed.html
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
///     packing = Aligned,
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
/// ### Packed struct example
///
/// This demonstrates how you can get a pointer to a field from a pointer to
/// a packed struct (it's UB to use references to fields here).
///
/// ```rust
/// use repr_offset::{unsafe_struct_field_offsets, Packed};
///
/// let mut bar = Bar{ mugs: 3, bottles: 5, table: "wooden".to_string() };
///
/// assert_eq!( replace_table(&mut bar, "metallic".to_string()), "wooden".to_string());
/// assert_eq!( replace_table(&mut bar, "granite".to_string()), "metallic".to_string());
/// assert_eq!( replace_table(&mut bar, "carbonite".to_string()), "granite".to_string());
///
/// fn replace_table(this: &mut Bar, replacement: String)-> String{
///     let ptr = Bar::OFFSET_TABLE.get_raw_mut(this);
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
///     packing = Packed,
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
        packing = $packing:ty,

        impl[ $($impl_params:tt)* ] $self:ty
        $(where [ $($where:tt)* ])?
        {
            $( $vis:vis const $offset:ident : $field_ty:ty; )*
        }
    )=>{
        impl<$($impl_params)*>  $self
        $(where $($where)*)?
        {
            $crate::_priv_unsafe_struct_field_offsets_inner!{
                Self( $($Self,)? Self, )
                packing = $packing,
                previous($crate::FieldOffset::<_,(),_>::new(0), $(Self::$offset,)*)
                offsets($($vis $offset: $field_ty;)*)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_unsafe_struct_field_offsets_inner{
    (
        Self( $Self:ty, $($_ignored_Self:ty,)? )
        packing = $packing:ty,
        previous( $prev_offset:expr, $($prev:tt)* )
        offsets(
            $vis:vis $offset:ident : $field_ty:ty;
            $($next:tt)*
        )
    )=>{
        $vis const $offset: $crate::FieldOffset<$Self,$field_ty,$packing> = unsafe{
            $prev_offset.next_field_offset()
        };
        $crate::_priv_unsafe_struct_field_offsets_inner!{
            Self($Self,)
            packing = $packing,
            previous($($prev)*)
            offsets($($next)*)
        }
    };
    (
        Self( $Self:ty, $($_ignored_Self:ty,)? )
        packing = $packing:ty,
        previous($($prev:tt)*)
        offsets()
    )=>{};
}
