use crate::FieldOffset;

use core::marker::PhantomData;

mod tuple_impls;

//////////////////////////////////////////////////////////////////////////////////

pub unsafe trait GetFieldOffset<K>: Sized {
    type This;
    type Field;
    type Alignment;

    const PRIV_OFFSET: PrivateFieldOffset<Self, K, Self::This, Self::Field, Self::Alignment>;
}

//////////////////////////////////////////////////////////////////////////////////

pub struct PrivateFieldOffset<AC, K, S, F, A> {
    offset: FieldOffset<S, F, A>,
    _associated_consts_from: PhantomData<fn() -> (AC, K)>,
    // The type that we got this PrivateFieldOffset from,
    // not necessarily same as the one that contains the field,
    // that is `S`.
    #[doc(hidden)]
    pub ac: PhantomData<fn() -> AC>,
}

impl<AC, K, S, F, A> Copy for PrivateFieldOffset<AC, K, S, F, A> {}

impl<AC, K, S, F, A> Clone for PrivateFieldOffset<AC, K, S, F, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<AC, K, S, F, A> PrivateFieldOffset<AC, K, S, F, A> {
    pub const fn new(offset: FieldOffset<S, F, A>) -> Self {
        Self {
            offset,
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
            ac: crate::utils::MakePhantomData::FN_RET,
        }
    }

    pub const unsafe fn private_field_offset(self) -> FieldOffset<S, F, A> {
        self.offset
    }
}

impl<K, S, F, A> PrivateFieldOffset<S, K, S, F, A> {
    #[doc(hidden)]
    pub const fn infer(self, _struct: &S) {}
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub type PrivateFieldOffsetSameType<K, S, F, A> = PrivateFieldOffset<S, K, S, F, A>;

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub fn loop_create_mutref<'a, S>(_: PhantomData<fn() -> S>) -> &'a mut S {
    loop {}
}

#[doc(hidden)]
pub fn loop_create_fo<S, F, A>(_: PhantomData<fn() -> S>) -> FieldOffset<S, F, A> {
    loop {}
}

#[doc(hidden)]
pub fn loop_create_val<S>(_: PhantomData<fn() -> S>) -> S {
    loop {}
}

////////////////////////////////////////////////////////////////////////////////
