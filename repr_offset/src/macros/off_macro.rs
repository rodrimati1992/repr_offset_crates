/// Gets the [`FieldOffset`] for the passed in type and (possibly nested) field.
///
/// This macro allows accessing private fields, following the normal Rust rules around privacy.
///
/// This macro doesn't allow accessing fields from type parameters in generic functions,
/// for that you can use `PUB_OFF`,
/// but it can only get the [`FieldOffset`] for public fields.
///
/// # Type Argument
///
/// The type parameter can be passed as either:
///
/// - The path to the type (including the type name)
///
/// - A type with generic arguments
///
/// ### Caveats with path argument
///
/// With the path way of passing the type,
/// if the type has all type parameters defaulted or is otherwise generic,
/// there can be type inference issues.
///
/// To fix type inference issues with defaulted types,
/// you can write `<>` (eg: `OFF!(for_examples::ReprC<>; a.b)`).
///
/// If a generic type is passed, and its arguments can be inferred from context,
/// it's only necessary to specify the type of the accessed field,
/// otherwise you need to write the full type.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     OFF,
///     FieldOffset, ROExtAcc,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// // Passing the type as a path
/// assert_eq!(OFF!(ReprC; a).get(&this), &this.a);
///
/// // Passing the type as a type
/// assert_eq!(OFF!(ReprC<_, _, _, _>; b).get(&this), &this.b);
///
/// // Passing the type as a path
/// assert_eq!(this.f_get(OFF!(ReprC; c)), &this.c);
///
/// // Passing the type as a type
/// assert_eq!(this.f_get(OFF!(ReprC<_, _, _, _>; d)), &this.d);
/// ```
///
/// [`FieldOffset`]: ./struct.FieldOffset.html
#[macro_export]
macro_rules! OFF{
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ;
        $($fields:tt).+
    )=>{
        $crate::__priv_OFF_path!(
            [$(:: $($leading)?)? $first $(::$trailing)* ];
            $($fields).+
        )
    };
    ($type:ty; $($fields:tt).+ )=>{unsafe{
        let marker =  $crate::utils::MakePhantomData::<$type>::FN_RET;

        $crate::pmr::FOAssertStruct{
            offset:{
                use $crate::get_field_offset::r#unsafe::unsafe_get_private_field;
                unsafe_get_private_field::<
                    _,
                    $crate::tstr::TS!($($fields),*)
                >::__unsafe__GET_PRIVATE_FIELD_OFFSET
            },
            struct_: {
                let _ = move || {
                    let variable = $crate::pmr::loop_create_mutref(marker);
                    #[allow(unused_unsafe)]
                    unsafe{ let _ = (*variable) $(.$fields)*; }
                };
                marker
            },
        }.offset
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_OFF_path{
    ([$($path:tt)*]; $($fields:tt).+)=>{
        $crate::pmr::FOAssertStruct{
            offset:{
                use $crate::get_field_offset::r#unsafe::unsafe_get_private_field;
                unsafe_get_private_field::<
                    _,
                    $crate::tstr::TS!($($fields),*)
                >::__unsafe__GET_PRIVATE_FIELD_OFFSET
            },
            struct_: {
                use $crate::utils::AsPhantomData;
                let marker = $($path)*::__REPR_OFFSET_PHANTOMDATA_FN;
                let _ = move || {
                    let variable = $crate::pmr::loop_create_mutref(marker);
                    #[allow(unused_unsafe)]
                    unsafe{ let _ = (*variable) $(.$fields)*; }
                };
                marker
            },
        }.offset
    }
}

/// Gets the [`FieldOffset`] for a (possibly nested) field, and an optionally passed in value.
///
/// The value argument is only necessary when the type that the fields are
/// from can't be inferred.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     off,
///     FieldOffset, ROExtAcc,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// // The value must be passed to the macro when you want to call
/// // any method on the returned `FieldOffset`.
/// assert_eq!(off!(this; a).get(&this), &this.a);
///
/// assert_eq!(this.f_get(off!(this; b)), &this.b);
///
/// assert_eq!(this.f_get(off!(c)), &this.c);
/// assert_eq!(this.f_get(off!(d)), &this.d);
/// ```
///
/// [`FieldOffset`]: ./struct.FieldOffset.html
#[macro_export]
macro_rules! off{
    ($value:expr; $($fields:tt).+ )=>{
        $crate::pmr::FOAssertStruct{
            offset:{
                use $crate::get_field_offset::r#unsafe::unsafe_get_private_field;
                unsafe_get_private_field::<
                    _,
                    $crate::tstr::TS!($($fields),*)
                >::__unsafe__GET_PRIVATE_FIELD_OFFSET
            },
            struct_: {
                use $crate::utils::AsPhantomData;
                let mut marker = $crate::pmr::PhantomData;
                if false {
                    let _ = $crate::utils::AsPhantomDataFn{
                        reference: &$value,
                        ty: marker,
                    };
                    let variable = $crate::pmr::loop_create_mutref(marker);
                    #[allow(unused_unsafe)]
                    unsafe{ let _ = (*variable) $(.$fields)*; }
                }
                marker
            },
        }.offset
    };
    ( $($fields:tt).+ )=>{{
        let marker = $crate::pmr::PhantomData;

        if false {
            $crate::pmr::loop_create_fo(marker)
        } else {
            let _ = || {
                let value = $crate::pmr::loop_create_val(marker);
                #[allow(unused_unsafe)]
                unsafe{ let _ = value $(.$fields)*; }
            };

            type __Key = $crate::tstr::TS!($($fields),*);

            use $crate::get_field_offset::r#unsafe::unsafe_get_private_field;

            unsafe_get_private_field::<_,__Key>::__unsafe__GET_PRIVATE_FIELD_OFFSET
        }
    }};
}

/// Gets the [`FieldOffset`] for a (possibly nested) public field,
/// and an optionally passed in value.
///
/// This is the same as the [`off`] macro,
/// except that this can't access private fields,
/// and it allows accessing fields from type parameters in generic functions.
///
/// The value argument is only necessary when the type that the fields are
/// from can't be inferred.
///
/// # Examples
///
/// ### Named Type
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     pub_off,
///     FieldOffset, ROExtAcc,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// // The value must be passed to the macro when you want to call
/// // any method on the returned `FieldOffset`.
/// assert_eq!(pub_off!(this; a).get(&this), &this.a);
///
/// assert_eq!(this.f_get(pub_off!(this; b)), &this.b);
///
/// assert_eq!(this.f_get(pub_off!(c)), &this.c);
/// assert_eq!(this.f_get(pub_off!(d)), &this.d);
/// ```
///
/// ### Accessing fields from type parameters
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     tstr::TS,
///     pub_off,
///     FieldOffset, GetPubFieldOffset, ROExtOps,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// assertions(this);
///
/// fn assertions<T, A>(this: T)
/// where
///     T: GetPubFieldOffset<TS!(a), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(b), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(c), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(d), Type = u8, Alignment = A>,
///     T: ROExtOps<A>,
/// {
///     assert_eq!(this.f_get_copy(pub_off!(this; a)), 3);
///     assert_eq!(this.f_get_copy(pub_off!(this; b)), 5);
///     assert_eq!(this.f_get_copy(pub_off!(c)), 8);
///     assert_eq!(this.f_get_copy(pub_off!(d)), 13);
/// }
/// ```
///
/// [`off`]: ./macro.off.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
///
#[macro_export]
macro_rules! pub_off{
    ($value:expr; $($fields:tt).+ )=>{
        $crate::pmr::FOAssertStruct{
            offset: $crate::pmr::GetPubFieldOffset::<$crate::tstr::TS!($($fields),*)>::OFFSET,
            struct_: {
                let mut marker = $crate::pmr::PhantomData;
                if false {
                    let _ = $crate::utils::AsPhantomDataFn{
                        reference: &$value,
                        ty: marker,
                    };
                }
                marker
            },
        }.offset
    };
    ( $($fields:tt).+ )=>{
        $crate::pmr::GetPubFieldOffset::<$crate::tstr::TS!($($fields),*)>::OFFSET
    };
}

/// Gets the [`FieldOffset`] for the passed in type and (possibly nested) public field.
///
/// This is the same as the [`OFF`] macro,
/// except that this can't access private fields,
/// and it allows accessing fields from type parameters in generic functions.
///
/// # Examples
///
/// ### Named Type
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     PUB_OFF,
///     FieldOffset, ROExtAcc,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// // Passing the type as a path
/// assert_eq!(PUB_OFF!(ReprC; a).get(&this), &this.a);
///
/// // Passing the type as a type
/// assert_eq!(PUB_OFF!(ReprC<_, _, _, _>; b).get(&this), &this.b);
///
/// // Passing the type as a path
/// assert_eq!(this.f_get(PUB_OFF!(ReprC; c)), &this.c);
///
/// // Passing the type as a type
/// assert_eq!(this.f_get(PUB_OFF!(ReprC<_, _, _, _>; d)), &this.d);
/// ```
///
///
/// ### Accessing fields from type parameters
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     tstr::TS,
///     PUB_OFF,
///     FieldOffset, GetPubFieldOffset, ROExtOps,
/// };
///
/// let this = ReprC {a: 3u8, b: 5u8, c: 8u8, d: 13u8};
///
/// assertions(this);
///
/// fn assertions<T, A>(this: T)
/// where
///     T: GetPubFieldOffset<TS!(a), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(b), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(c), Type = u8, Alignment = A>,
///     T: GetPubFieldOffset<TS!(d), Type = u8, Alignment = A>,
///     T: ROExtOps<A>,
/// {
///     assert_eq!(this.f_get_copy(PUB_OFF!(T; a)), 3);
///     assert_eq!(this.f_get_copy(PUB_OFF!(T; b)), 5);
///     assert_eq!(this.f_get_copy(PUB_OFF!(T; c)), 8);
///     assert_eq!(this.f_get_copy(PUB_OFF!(T; d)), 13);
/// }
/// ```
///
/// [`OFF`]: ./macro.OFF.html
/// [`FieldOffset`]: ./struct.FieldOffset.html
#[macro_export]
macro_rules! PUB_OFF{
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ;
        $($fields:tt).+
    )=>{
        $crate::__priv_ty_PUB_OFF_path!(
            [$(:: $($leading)?)? $first $(::$trailing)* ];
            $($fields).+
        )
    };
    ($type:ty; $($fields:tt).+ )=>{
        <$type as $crate::pmr::GetPubFieldOffset::<$crate::tstr::TS!($($fields),*)>>::OFFSET
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __priv_ty_PUB_OFF_path{
    ([$($path:tt)*]; $($fields:tt).+)=>{
        $crate::pmr::FOAssertStruct{
            offset: $crate::pmr::GetPubFieldOffset::<$crate::tstr::TS!($($fields),*)>::OFFSET,
            struct_: {
                use $crate::utils::AsPhantomData;
                $($path)*::__REPR_OFFSET_PHANTOMDATA_FN
            }
        }.offset
    }
}
