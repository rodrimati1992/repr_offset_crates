/// Declares a sequence of (associated) constants with the offset of the listed fields.
///
/// # Parameters
///
/// The `Self` parameter is the type that contains the fields,
/// possibly a different type to the one that this declares (associated) constants for.
///
/// The `alignment` parameter can be either [`Aligned`] or [`Unaligned`],
/// and describes whether the fields are aligned or potentially unaligned,
/// changing how the field is accessed in [`FieldOffset`] methods.
///
///
/// [`Aligned`]: ./struct.Aligned.html
/// [`Unaligned`]: ./struct.Unaligned.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
#[macro_export]
macro_rules! unsafe_offset_constants{
    (
        Self = $Self:ty,
        alignment = $alignment:ty,
        associated_constants = $associated_constants:ident,
        offsets = [
            $($offset:ident : $field_ty:ty, )*
            $(,)?
        ]$(,)?
    )=>{
        const INITIAL: $crate::FieldOffset<$Self,(),$alignment> = unsafe{ FieldOffset::new(0) };
        $crate::unsafe_offset_constants!{
            @inner
            Self = $Self,
            alignment = $alignment,
            associated_constants = $associated_constants,
            previous(INITIAL, $($offset,)*)
            fields($($offset: $field_ty,)*)
        }
    };
    (@prev_offset true, $prev_offset:ident $(,)?)=>{
        Self::$prev_offset
    };
    (@prev_offset false, $prev_offset:ident $(,)?)=>{
        $prev_offset
    };
    (@inner
        Self = $Self:ty,
        alignment = $alignment:ty,
        associated_constants = $associated_constants:ident,
        previous( $prev_offset:ident, $($prev:tt)* )
        fields(
            $offset:ident : $field_ty:ty,
            $($next:tt)*
        )
    )=>{
        const $offset: $crate::FieldOffset<$Self,$field_ty,$alignment> = unsafe{
            $crate::FieldOffset::next_field_offset(
                $crate::unsafe_offset_constants!(
                    @prev_offset
                    $associated_constants,
                    $prev_offset,
                )
            )
        };
        $crate::unsafe_offset_constants!{
            @inner
            Self = $Self,
            alignment = $alignment,
            associated_constants = $associated_constants,
            previous($($prev)*)
            fields($($next)*)
        }
    };
    (@inner
        Self = $Self:ty,
        alignment = $alignment:ty,
        associated_constants = $associated_constants:ident,
        previous($($prev:tt)*)
        fields()
    )=>{};
}
