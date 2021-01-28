/// Declares a sequence of associated constants with the offsets of the listed fields,
/// and implements the [`GetFieldOffset`] trait.
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
/// Note that passing the this parameter unconditionally causes the type
/// not to implement [`GetFieldOffset`].
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
/// ### `impl_GetFieldOffset`
///
/// The optional `impl_GetFieldOffset` parameter determines whether `$self`
/// implements the [`GetFieldOffset`] trait,
/// which allows getting the [`FieldOffset`] for each field using the
/// [`OFF`](./macro.OFF.html),
/// [`off`](./macro.off.html),
/// [`PUB_OFF`](./macro.PUB_OFF.html), and
/// [`pub_off`](./macro.pub_off.html) macros.
///
/// The valid values for this parameter are:
/// - (not passing this parameter): Implements [`GetFieldOffset`].
/// - `false`: Does not implement [`GetFieldOffset`].
/// - `true`: Implements [`GetFieldOffset`].
///
/// Note that passing the `Self` parameter unconditionally causes the type
/// not to implement [`GetFieldOffset`].
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
///     // Optional parameter.
///     impl_GetFieldOffset = false,
///
///     impl[T: Copy, U] Bar<T, U>
///     where[ U: Clone ]
///     {
///         pub const OFFSET_0, 0: T;
///         pub const OFFSET_1, 1: U;
///     }
/// }
///
/// ```
///
/// ### Unaligned struct example
///
/// This example demonstrates how you can replace fields in a packed struct,
/// as well as getting a raw pointer to a field,
/// using both [`FieldOffset`] methods, and extension traits from the [`ext`] module
///
/// ```rust
/// use repr_offset::{
///     unsafe_struct_field_offsets,
///     off,
///     ROExtAcc, ROExtOps, Unaligned,
/// };
///
/// // Replacing the table field
/// {
///     let mut bar = Bar{ mugs: 3, bottles: 5, table: "wooden".to_string() };
///    
///     assert_eq!( replace_table_a(&mut bar, "metallic".to_string()), "wooden".to_string());
///     assert_eq!( replace_table_b(&mut bar, "granite".to_string()), "metallic".to_string());
///     assert_eq!( replace_table_b(&mut bar, "carbonite".to_string()), "granite".to_string());
///     
///     fn replace_table_a(this: &mut Bar, replacement: String)-> String{
///         Bar::OFFSET_TABLE.replace_mut(this, replacement)
///     }
///    
///     fn replace_table_b(this: &mut Bar, replacement: String)-> String{
///         this.f_replace(off!(table), replacement)
///     }
/// }
///
/// // Getting raw pointers to fields
/// unsafe {
///     let bar = Bar{ mugs: 3, bottles: 5, table: "wooden".to_string() };
///
///     assert_eq!(get_mugs_ptr(&bar).read_unaligned(), 3);
///
///     assert_eq!(get_bottles_ptr(&bar).read_unaligned(), 5);
///
///     fn get_mugs_ptr(this: &Bar) -> *const u32 {
///         Bar::OFFSET_MUGS.get_ptr(this)
///     }
///    
///     fn get_bottles_ptr(this: &Bar) -> *const u16 {
///         this.f_get_ptr(off!(bottles))
///     }
/// }
///
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
///         pub const OFFSET_MUGS, mugs: u32;
///         pub const OFFSET_BOTTLES, bottles: u16;
///         pub const OFFSET_TABLE, table: String;
///     }
/// }
///
/// ```
///
/// [`Aligned`]: ./alignment/struct.Aligned.html
/// [`Unaligned`]: ./alignment/struct.Unaligned.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
/// [`GetFieldOffset`]: ./get_field_offset/trait.GetFieldOffset.html
///
/// [`FieldOffset`]: ./struct.FieldOffset.html
/// [`ext`]: ./ext/index.html
///
#[macro_export]
macro_rules! unsafe_struct_field_offsets{
    (
        $( Self = $Self:ty, )?
        alignment =  $alignment:ty,
        $( usize_offsets = $usize_offsets:ident,)?
        $( impl_GetFieldOffset = $impl_gfo:ident,)?

        $(#[$impl_attr:meta])*
        impl[ $($impl_params:tt)* ] $self:ty
        $(where [ $($where:tt)* ])?
        {
            $(
                $(#[$const_attr:meta])*
                $( pub $(($($inn:tt)*))? )?
                const $offset:ident, $field_ident:tt: $field_ty:ty;
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
                    impl_GetFieldOffset( $(false, $Self:ty )? $($impl_gfo,)? true,)

                    $(#[$impl_attr])*
                    impl[ $($impl_params)* ] $self
                    where [ $($($where)*)? ]
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
                    ($( pub $(($($inn)*))? )?) $offset, $field_ident: $field_ty;
                )*)
            }
        }

        $crate::expand_if_true!{[ $(false $Self:ty )? $($impl_gfo)? true,]
            $(#[$impl_attr])*
            unsafe impl<$($impl_params)*> $crate::pmr::ImplsGetFieldOffset for $self
            $(where $($where)*)?
            {}
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
    (@FieldOffsetWithVis false, $expr:expr)=>{
        $crate::pmr::FieldOffsetWithVis::from_fieldoffset($expr)
    };
    (@FieldOffsetWithVis true, $expr:expr)=>{unsafe{
        $crate::pmr::FieldOffsetWithVis::new($expr)
    }};

    (
        params $params:tt
        params(
            Self( $Self:ty, $($_ignored_Self:ty,)? )
            alignment =  $alignment:ty,
            usize_offsets($usize_offsets:ident, $($_ignored_io:ident,)? )
            impl_GetFieldOffset($impl_gfo:ident, $($_ignored_impl_gfo:tt)*)

            $(#[$impl_attr:meta])*
            impl[ $($impl_params:tt)* ] $self:ty
            where [ $($where:tt)* ]
        )
        previous( ($prev_offset:expr, $prev_ty:ty), $($prev:tt)* )
        offsets(
            $(#[$const_attr:meta])*
            ($($vis:tt)*) $offset:ident, $field_ident:tt : $field_ty:ty;
            $($next:tt)*
        )
    )=>{
        $(#[$const_attr])*
        $($vis)* const $offset:
            $crate::_priv_usfoi!(
                @ty $usize_offsets, $Self, $field_ty, $alignment
            )
        = unsafe{
            $crate::_priv_impl_getfieldoffset!{
                impl_GetFieldOffset = $impl_gfo,
                Self = $Self,
                alignment = $alignment,
                usize_offsets = $usize_offsets,

                $(#[$impl_attr])*
                impl[ $($impl_params)* ] $self
                where [ $($where)* ]

                (($($vis)*), $offset, $field_ident : $field_ty)
            }

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

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_impl_getfieldoffset{
    (
        impl_GetFieldOffset = true,

        Self = $Self:ty,
        alignment = $alignment:ty,
        usize_offsets = $usize_offsets:ident,

        $(#[$impl_attr:meta])*
        impl[ $($impl_params:tt)* ] $self:ty
        where [ $($where:tt)* ]

        (($($vis:tt)*), $offset:ident, $field_ident:tt : $field_ty:ty)

    )=>{
        type __Key = $crate::tstr::TS!($field_ident);
        type __Privacy = $crate::_priv_get_privacy!($($vis)*);

        $crate::_priv_doc_attribute!{
            [$($vis)*]
            $(#[$impl_attr])*
            unsafe impl<$($impl_params)*> $crate::pmr::GetFieldOffset<__Key> for $self
            where $($where)*
            {
                type Type = $field_ty;
                type Alignment = $alignment;
                type Privacy = __Privacy;

                const OFFSET_WITH_VIS: $crate::pmr::FieldOffsetWithVis<
                    Self,
                    __Privacy,
                    __Key,
                    Self::Type,
                    $alignment,
                > = unsafe{
                    $crate::_priv_usfoi!( @FieldOffsetWithVis $usize_offsets, Self::$offset)
                };
            }
        }
    };
    (impl_GetFieldOffset = false $($rem:tt)*)=>[];
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_get_privacy {
    (pub) => {
        $crate::privacy::IsPublic
    };
    ($($vis:tt)*) => {
        $crate::privacy::IsPrivate
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _priv_doc_attribute {
    ([pub] $($item:tt)*) => {
        $($item)*
    };
    ([$($vis:tt)*] $($item:tt)*) => {
        #[doc(hidden)]
        $($item)*
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_if_true {
    ([true $($ignore:tt)*] $($item:tt)*) => {
        $($item)*
    };
    ([false $($ignore:tt)*] $($item:tt)*) => {};
}
