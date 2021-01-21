use crate::{privacy::IsPublic, FieldOffset};

use core::marker::PhantomData;

mod tuple_impls;

//////////////////////////////////////////////////////////////////////////////////

/// For getting the offset of a field given its name.
///
/// This trait exists to make it possible for the [`Off`] and [`off`] macros to get the
/// [`FieldOffset`] of a field.
///
/// # SemVer
///
/// Impls of this trait where the `Privacy` associated type is `Private`
/// can change or be removed in semver compatible versions,
///
/// Prefer using the [`GetPubFieldOffset`] trait alias for bounds instead,
/// since that is for public fields.
///
/// # Type Parameter
///
/// The `FN` type parameter is the path to the field that this gets the offset for, it can be:
///
/// - A [`tstr::TStr`]: representing a single field, eg: (`tstr::TS!(foo)`).
///
/// - A tuple of [`tstr::TStr`]s: representing a nested field, eg: (`tstr::TS!(foo,bar,baz)`).
///
/// [`Off`] ../macro.Off.html
/// [`off`]: ../macro.off.html
/// [`FieldOffset`]: ../struct.FieldOffset.html
/// [`GetPubFieldOffset`]: ../struct.GetPubFieldOffset.html
///
pub unsafe trait GetFieldOffset<FN>: Sized {
    /// The type of the field.
    type Field;
    /// Whether the field is [`Aligned`] or [`Unaligned`].
    ///
    /// [`Aligned`]: ../alignment/struct.Aligned.html
    /// [`Unaligned`]: ../alignment/struct.Unaligned.html
    type Alignment;

    /// Whether the field is private or not, either `[`IsPublic`]` or [`IsPrivate`].
    ///
    /// [`IsPublic`]: ../privacy/struct.IsPublic.html
    /// [`IsPrivate`]: ../privacy/struct.IsPrivate.html
    type Privacy;

    /// For initializing the `OFFSET_WITH_VIS` associated constant,
    /// this is the offset of the field.
    const INIT_OFFSET_WITH_VIS: InitPrivOffset<
        Self,
        Self::Privacy,
        FN,
        Self::Field,
        Self::Alignment,
    >;

    /// The offset of the field,
    /// wrapped in a `FieldOffsetWithVis` because the field may te private.
    const OFFSET_WITH_VIS: FieldOffsetWithVis<
        Self,
        Self::Privacy,
        FN,
        Self::Field,
        Self::Alignment,
    > = <Self as GetFieldOffset<FN>>::INIT_OFFSET_WITH_VIS.to_private_field_offset();
}

//////////////////////////////////////////////////////////////////////////////////

/// An alias of the [`GetFieldOffset`] trait for public fields.
///
/// [`GetFieldOffset`]: ./trait.GetFieldOffset.html
pub trait GetPubFieldOffset<FN>: GetFieldOffset<FN, Privacy = IsPublic> {
    /// An alias for `GetFieldOffset::Field`
    type PubField;

    /// The offset of the field.
    const OFFSET: FieldOffset<Self, Self::Field, Self::Alignment> =
        <Self as GetFieldOffset<FN>>::OFFSET_WITH_VIS.to_field_offset();
}

impl<FN, Ty> GetPubFieldOffset<FN> for Ty
where
    Ty: GetFieldOffset<FN, Privacy = IsPublic>,
{
    type PubField = <Ty as GetFieldOffset<FN>>::Field;
}

//////////////////////////////////////////////////////////////////////////////////

/// Gets the type of the field in the `GetPubFieldOffset<FN>` impl for `This`.
pub type GetFieldType<This, FN> = <This as GetPubFieldOffset<FN>>::PubField;

//////////////////////////////////////////////////////////////////////////////////

/// An opaque wrapper around a [`FieldOffset`],
/// purely to initialize the [`GetFieldOffset::INIT_OFFSET_WITH_VIS`] associated constant.
///
/// [`GetFieldOffset::INIT_OFFSET_WITH_VIS`]:
/// ./trait.GetFieldOffset.html#associatedconstant.INIT_OFFSET_WITH_VIS
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
///
pub struct InitPrivOffset<S, V, FN, F, A> {
    offset: FieldOffset<S, F, A>,
    _associated_consts_from: PhantomData<fn() -> (FN, V)>,
}

impl<S, V, FN, F, A> InitPrivOffset<S, V, FN, F, A> {
    /// Constructs this `InitPrivOffset`
    pub const fn new(offset: FieldOffset<S, F, A>) -> Self {
        Self {
            offset,
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
        }
    }

    const fn to_private_field_offset(self) -> FieldOffsetWithVis<S, V, FN, F, A> {
        FieldOffsetWithVis {
            offset: self.offset,
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
            ac: crate::utils::MakePhantomData::FN_RET,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////

/// A wrapper around a [`FieldOffset`] with a visibility type parameter
/// (whether the field is pub or not).
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
///
pub struct FieldOffsetWithVis<S, V, FN, F, A> {
    offset: FieldOffset<S, F, A>,
    _associated_consts_from: PhantomData<fn() -> (FN, V)>,
    // The type that we got this FieldOffsetWithVis from,
    // not necessarily same as the one that contains the field,
    // that is `S`.
    #[doc(hidden)]
    pub ac: PhantomData<fn() -> S>,
}

impl<S, V, FN, F, A> Copy for FieldOffsetWithVis<S, V, FN, F, A> {}

impl<S, V, FN, F, A> Clone for FieldOffsetWithVis<S, V, FN, F, A> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<FN, S, F, A> FieldOffsetWithVis<S, IsPublic, FN, F, A> {
    /// Unwraps this into a [`FieldOffset`] for a public field.
    pub const fn to_field_offset(self) -> FieldOffset<S, F, A> {
        self.offset
    }
}

impl<S, V, FN, F, A> FieldOffsetWithVis<S, V, FN, F, A> {
    /// Unwraps this into a [`FieldOffset`] for a possibly private field.
    ///
    /// # Safety
    ///
    /// Because the field may be private, modifying its state could cause undefined behavior,
    /// and is only supposed to be done in a context where the field is accessible.
    ///
    /// [`FieldOffset`]: ../struct.FieldOffset.html
    ///
    pub const unsafe fn private_field_offset(self) -> FieldOffset<S, F, A> {
        self.offset
    }

    #[doc(hidden)]
    pub const fn infer(self, _struct: &S) {}
}

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