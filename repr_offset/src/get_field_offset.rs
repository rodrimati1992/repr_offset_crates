use crate::{privacy::IsPublic, FieldOffset};

use core::marker::PhantomData;

mod tuple_impls;

//////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement `GetFieldOffset`.
///
/// This is only required as a workaround for the time that `cargo doc` takes to run.
pub unsafe trait ImplsGetFieldOffset: Sized {}

//////////////////////////////////////////////////////////////////////////////////

/// Helper type to implement `GetFieldOffset<(N0, N1, ...)>` for all types without
/// blowing up the time that `cargo doc` takes to run.
pub struct ImplGetNestedFieldOffset<T>(T);

//////////////////////////////////////////////////////////////////////////////////

/// For getting the offset of a field given its name.
///
/// This trait exists to make it possible for the
/// [`OFF`](./macro.OFF.html),
/// [`off`](./macro.off.html),
/// [`PUB_OFF`](./macro.PUB_OFF.html), and
/// [`pub_off`](./macro.pub_off.html) macros
/// to get the [`FieldOffset`] of a field.
///
/// This trait is by default implemented by the [`unsafe_struct_field_offsets`] macro,
/// and [`ReprOffset`] derive macro.
///
/// [`unsafe_struct_field_offsets`]: ../macro.unsafe_struct_field_offsets.html
/// [`ReprOffset`]: ../derive.ReprOffset.html
///
/// # Safety
///
/// ### Non-nested fields
///
/// Implementors must ensure that for any given impl of `GetFieldOffset<TS!(<field_name>)>`
/// for a type there is a `<field_name>` field stored inline in the type,
/// accessible through `.<field_name>`.
///
/// Implementors must ensure that the
/// [`OFFSET_WITH_VIS`](#associatedconstant.OFFSET_WITH_VIS)
/// associated constant contains the [`FieldOffset`] for the `<field_name>` field,
/// with the correct offset(in bytes), field type, and alignment type parameter.
///
/// Implementors must ensure that there is the only one impl of
/// `GetFieldOffset` for that type through
/// which one can get the [`FieldOffset`] for the `<field_name>` field,
///
/// `<field_name>` is used here to refer to any one field (eg: a field named `foo`),
/// all mentions of that field in this section refer to the same field.
///
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
/// # Example
///
/// ### Manual Implementation
///
/// This example demonstrates how you can implement `GetFieldOffset` manually.
///
/// ```rust
/// use repr_offset::{
///     alignment::{Aligned, Unaligned},
///     get_field_offset::{GetFieldOffset, FieldOffsetWithVis as FOWithVis},
///     privacy::{IsPublic, IsPrivate},
///     tstr::TS,
///     off, pub_off,
///     FieldOffset, ROExtAcc, ROExtOps,
/// };
///
/// #[repr(C, packed)]
/// struct Foo {
///     wheel_count: u8,
///     pub(crate) seat_size: u128,
///     pub is_automatic: bool,
/// }
///
/// let foo = Foo {
///     wheel_count: 3,
///     seat_size: 5,
///     is_automatic: false,
/// };
///
/// // We can get a reference because the field is aligned.
/// assert_eq!(foo.f_get(off!(wheel_count)), &3);
///
/// // The seat_size field is unaligned inside of Foo, so we can't get a reference.
/// assert_eq!(foo.f_get_copy(off!(seat_size)), 5);
///
/// // We can get a reference because the field is aligned.
/// //
/// // Also, because the field is public, you can use `pub_off` to get its FieldOffset.
/// assert_eq!(foo.f_get(pub_off!(is_automatic)), &false);
///
///
/// unsafe impl GetFieldOffset<TS!(wheel_count)> for Foo {
///     type Type = u8;
///     type Alignment = Aligned;
///     type Privacy = IsPrivate;
///     
///     const OFFSET_WITH_VIS: FOWithVis<Self, IsPrivate, TS!(wheel_count), u8, Aligned> = unsafe {
///         FOWithVis::new(0)
///     };
/// }
///
/// unsafe impl GetFieldOffset<TS!(seat_size)> for Foo {
///     type Type = u128;
///     type Alignment = Unaligned;
///     type Privacy = IsPrivate;
///     
///     const OFFSET_WITH_VIS: FOWithVis<Self, IsPrivate, TS!(seat_size), u128, Unaligned> =
///         unsafe{
///             <Self as GetFieldOffset<TS!(wheel_count)>>::OFFSET_WITH_VIS
///                 .private_field_offset()
///                 .next_field_offset()
///                 .with_vis()
///         };
/// }
///
/// unsafe impl GetFieldOffset<TS!(is_automatic)> for Foo {
///     type Type = bool;
///     type Alignment = Aligned;
///     type Privacy = IsPublic;
///     
///     const OFFSET_WITH_VIS: FOWithVis<Self, IsPublic, TS!(is_automatic), bool, Aligned> =
///         unsafe{
///             <Self as GetFieldOffset<TS!(seat_size)>>::OFFSET_WITH_VIS
///                 .private_field_offset()
///                 .next_field_offset()
///                 .with_vis()
///         };
/// }
///
/// ```
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
///
/// [`GetPubFieldOffset`]: ../struct.GetPubFieldOffset.html
///
///
pub unsafe trait GetFieldOffset<FN>: Sized {
    /// The type of the field.
    type Type;
    /// Whether the field is:
    ///
    /// - [`Aligned`]: `Self` has an alignment greater than or equal to the field type,
    /// usually when `Self` has one of the
    /// `#[repr(C)]`/`#[repr(C, align(...))]`/`#[repr(transparent)]` representations.
    ///
    /// - [`Unaligned`]: `Self` has an alignment smaller than the field type,
    /// usually when `Self` has the `#[repr(C, packed)]` representation.
    ///
    /// [`Aligned`]: ../alignment/struct.Aligned.html
    /// [`Unaligned`]: ../alignment/struct.Unaligned.html
    type Alignment;

    /// Whether the field is private or not, either:
    ///
    /// - `[`IsPublic`]`: When the field is `pub`.
    ///
    /// - [`IsPrivate`]: When the field has the default (private) visibility,
    /// or has a visibility smaller or equal to `pub(crate)`.
    ///
    /// [`IsPublic`]: ../privacy/struct.IsPublic.html
    /// [`IsPrivate`]: ../privacy/struct.IsPrivate.html
    type Privacy;

    /// The offset of the field.
    const OFFSET_WITH_VIS: FieldOffsetWithVis<
        Self,
        Self::Privacy,
        FN,
        Self::Type,
        Self::Alignment,
    >;
}

//////////////////////////////////////////////////////////////////////////////////

/// An alias of the [`GetFieldOffset`] trait for public fields.
///
/// [`GetFieldOffset`]: ./trait.GetFieldOffset.html
pub trait GetPubFieldOffset<FN>: GetFieldOffset<FN, Privacy = IsPublic> {
    /// An alias for `GetFieldOffset::Type`
    type PubType;

    /// An alias for `GetFieldOffset::Alignment`
    type PubAlignment;

    /// The offset of the field.
    const OFFSET: FieldOffset<Self, Self::Type, Self::Alignment> =
        <Self as GetFieldOffset<FN>>::OFFSET_WITH_VIS.to_field_offset();
}

impl<FN, Ty> GetPubFieldOffset<FN> for Ty
where
    Ty: GetFieldOffset<FN, Privacy = IsPublic>,
{
    type PubType = <Ty as GetFieldOffset<FN>>::Type;
    type PubAlignment = <Ty as GetFieldOffset<FN>>::Alignment;
}

//////////////////////////////////////////////////////////////////////////////////

/// Gets the type of a public field in the `GetPubFieldOffset<FN>` impl for `This`.
///
pub type FieldType<This, FN> = <This as GetPubFieldOffset<FN>>::PubType;

/// Gets the alignment of a public field in the `GetPubFieldOffset<FN>` impl for `This`.
pub type FieldAlignment<This, FN> = <This as GetPubFieldOffset<FN>>::PubAlignment;

////////////////////////////////////////////////////////////////////////////////

/// Gets the type of a (potentially) private field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the type of fields in types from external crates.
pub type PrivFieldType<This, FN> = <This as GetFieldOffset<FN>>::Type;

/// Gets the alignment of a (potentially) private field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the alignment of fields in types from external crates.
pub type PrivFieldAlignment<This, FN> = <This as GetFieldOffset<FN>>::Alignment;

/// Gets the privacy of a field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the privacy of fields in types from external crates.
pub type FieldPrivacy<This, FN> = <This as GetFieldOffset<FN>>::Privacy;

//////////////////////////////////////////////////////////////////////////////////

/// An wrapper around a [`FieldOffset`], with a visibility type parameter
/// (whether the field is pub or not).
///
/// # Type parameters
///
/// `S`: The type that contains the field
///
/// `V`: The visibility of the field, either [`IsPrivate`] or [`IsPublic`].
///
/// `FN`: The name of the field, written using the `repr_offset::tstr::TS` macro,
/// written as `TS!(field_name)`.
///
/// `F`: The type of the field.
///
/// `A`: The alignment of the field inside of `S`, either [`Aligned`] or [`Unaligned`].
///
///
/// [`GetFieldOffset::OFFSET_WITH_VIS`]:
/// ./trait.GetFieldOffset.html#associatedconstant.OFFSET_WITH_VIS
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
///
/// [`Aligned`]: ../alignment/struct.Aligned.html
/// [`Unaligned`]: ../alignment/struct.Unaligned.html
///
/// [`IsPublic`]: ../privacy/struct.IsPublic.html
/// [`IsPrivate`]: ../privacy/struct.IsPrivate.html
///
///
#[repr(transparent)]
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

impl<S, V, FN, F, A> FieldOffsetWithVis<S, V, FN, F, A> {
    /// Constructs this `FieldOffsetWithVis` from the `offset` (in bytes) of a field.
    ///
    /// # Safety
    ///
    /// This method has a superset of the safety requirements of [`FieldOffset::new`].
    ///
    /// The `FN` type parameter must be the name of the field using the
    /// `repr_offset::tstr::TS` macro,
    /// eg: `TS!(foo)` for the `foo` field.
    ///
    /// The `V` type parameter must be:
    /// - `[`IsPublic`]`: When the field is `pub`.
    ///
    /// - [`IsPrivate`]: When the field has the default (private) visibility,
    /// or has a visibility smaller or equal to `pub(crate)`.
    ///
    /// [`IsPublic`]: ../privacy/struct.IsPublic.html
    /// [`IsPrivate`]: ../privacy/struct.IsPrivate.html
    ///
    /// [`FieldOffset::new`]: ..//struct.FieldOffset.html#method.new
    /// [`FieldOffset`]: ../struct.FieldOffset.html
    pub const unsafe fn new(offset: usize) -> Self {
        Self {
            offset: FieldOffset::new(offset),
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
            ac: crate::utils::MakePhantomData::FN_RET,
        }
    }

    /// Constructs this `FieldOffsetWithVis` from `offset`.
    ///
    /// # Safety
    ///
    /// The `V` type parameter must be:
    /// - `[`IsPublic`]`: When the field is `pub`.
    ///
    /// - [`IsPrivate`]: When the field has the default (private) visibility,
    /// or has a visibility smaller or equal to `pub(crate)`.
    ///
    /// The `FN` type parameter must be the name of the field using the
    /// `repr_offset::tstr::TS` macro,
    /// eg: `TS!(foo)` for the `foo` field.
    ///
    /// [`IsPublic`]: ../privacy/struct.IsPublic.html
    /// [`IsPrivate`]: ../privacy/struct.IsPrivate.html
    ///
    /// [`FieldOffset::new`]: ..//struct.FieldOffset.html#method.new
    /// [`FieldOffset`]: ../struct.FieldOffset.html
    pub const unsafe fn from_fieldoffset(offset: FieldOffset<S, F, A>) -> Self {
        Self {
            offset,
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
            ac: crate::utils::MakePhantomData::FN_RET,
        }
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
    #[inline(always)]
    pub const unsafe fn private_field_offset(self) -> FieldOffset<S, F, A> {
        self.offset
    }

    /// Casts this `FieldOffsetWithVis` to be for a different struct.
    ///
    /// This is mostly useful for `#[repr(transparent)]` types to delegate to
    /// their single field.
    ///
    /// # Safety
    ///
    /// `SO` must contain a field of type `S` as its first field in memory,
    /// at offset 0.
    pub const unsafe fn cast_struct<SO>(self) -> FieldOffsetWithVis<SO, V, FN, F, A> {
        FieldOffsetWithVis {
            offset: FieldOffset::new(self.offset.offset()),
            _associated_consts_from: crate::utils::MakePhantomData::FN_RET,
            ac: crate::utils::MakePhantomData::FN_RET,
        }
    }

    #[doc(hidden)]
    #[inline(always)]
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

#[doc(hidden)]
pub mod r#unsafe {
    use super::*;

    #[allow(non_camel_case_types)]
    pub struct unsafe_get_private_field<S, FN>(S, FN);

    impl<S, FN> unsafe_get_private_field<S, FN>
    where
        S: GetFieldOffset<FN>,
    {
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub const __unsafe__GET_PRIVATE_FIELD_OFFSET: FieldOffset<S, S::Type, S::Alignment> =
            unsafe { <S as GetFieldOffset<FN>>::OFFSET_WITH_VIS.private_field_offset() };
    }
}
