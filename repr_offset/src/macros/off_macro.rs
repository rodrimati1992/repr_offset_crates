/// Gets the `FieldOffset` for the passed in type and (possibly nested) field.
#[macro_export]
macro_rules! OFF{
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ,
        $($fields:tt).+
    )=>{
        $crate::__priv_OFF_path!(
            [$(:: $($leading)?)? $first $(::$trailing)* ];
            $($fields).+
        )
    };
    ($type:ty, $($fields:tt).+ )=>{unsafe{
        let _ = |variable: &mut $type| {
            let _ = (*variable) $(.$fields)*;
        };

        <$type as $crate::pmr::GetFieldOffset::<$crate::tstr::TS!($($fields),*)>>::OFFSET_WITH_VIS
            .private_field_offset()
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

/// Gets the `FieldOffset` for the passed in value and (possibly nested) field.
#[macro_export]
macro_rules! off{
    ($value:expr, $($fields:tt).+ )=>{
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
            if false {
                let value = $crate::pmr::loop_create_val(marker);
                #[allow(unused_unsafe)]
                unsafe{ let _ = value $(.$fields)*; }
            }

            type __Key = $crate::tstr::TS!($($fields),*);

            use $crate::get_field_offset::r#unsafe::unsafe_get_private_field;

            unsafe_get_private_field::<_,__Key>::__unsafe__GET_PRIVATE_FIELD_OFFSET
        }
    }};
}

/// Gets the `FieldOffset` for the passed in value and (possibly nested) public field.
#[macro_export]
macro_rules! pub_off{
    ($value:expr, $($fields:tt).+ )=>{
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

/// Gets the `FieldOffset` for the passed in type and (possibly nested) public field.
#[macro_export]
macro_rules! PUB_OFF{
    (
        $(:: $(@$leading:tt@)? )? $first:ident $(:: $trailing:ident)* ,
        $($fields:tt).+
    )=>{
        $crate::__priv_ty_PUB_OFF_path!(
            [$(:: $($leading)?)? $first $(::$trailing)* ];
            $($fields).+
        )
    };
    ($type:ty, $($fields:tt).+ )=>{unsafe{
        let _ = |variable: &mut $type| {
            let _ = (*variable) $(.$fields)*;
        };

        <$type as $crate::pmr::GetPubFieldOffset::<$crate::tstr::TS!($($fields),*)>>::OFFSET
    }};
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
