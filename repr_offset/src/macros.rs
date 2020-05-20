/// Declares a sequence of associated constants with the offset of the listed fields.
///
/// # Parameters
///
/// The optional `Self` parameter overrides which struct the [`FieldOffset`] constants
/// are an offset inside of.
///
/// The `packing` parameter can be either [`Aligned`] or [`Packed`],
/// and describes whether the fields are aligned or potentially unaligned,
/// changing how the field is accessed in [`FieldOffset`] methods.
///
///
/// [`Aligned`]: ./struct.Aligned.html
/// [`Packed`]: ./struct.Packed.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
#[macro_export]
macro_rules! unsafe_offset_constants{
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
            $crate::_priv_unsafe_offset_constants_inner!{
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
macro_rules! _priv_unsafe_offset_constants_inner{
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
        $crate::_priv_unsafe_offset_constants_inner!{
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
