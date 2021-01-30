//! Trait for getting the `FieldOffset` of a field, and related items.
//!
//! One would implement the [`ImplsGetFieldOffset`] and [`GetFieldOffset`] traits,
//! and use [`GetPubFieldOffset`] as a bound.
//!
//! [`ImplsGetFieldOffset`]: ./trait.ImplsGetFieldOffset.html
//! [`GetFieldOffset`]: ./trait.GetFieldOffset.html
//! [`GetPubFieldOffset`]: ./trait.GetPubFieldOffset.html

use crate::{privacy::IsPublic, FieldOffset};

use core::marker::PhantomData;

mod tuple_impls;

//////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types that implement `GetFieldOffset`.
///
/// This trait is required for the `GetFieldOffset` impls that
/// get the [`FieldOffset`] of nested fields.
///
///
/// [`FieldOffset`]: ../struct.FieldOffset.html
///
/// This is only required as a workaround to lower the time that `cargo doc` takes to run.
pub unsafe trait ImplsGetFieldOffset: Sized {}

//////////////////////////////////////////////////////////////////////////////////

/// Hack use by `repr_offset` to implement `GetFieldOffset<(N0, N1, ...)>`
/// for all types without blowing up the time that `cargo doc` takes to run.
pub struct ImplGetNestedFieldOffset<T>(T);

//////////////////////////////////////////////////////////////////////////////////

/// For getting the offset of a field given its name.
///
/// This trait exists to make it possible for the
/// [`OFF!`], [`off`], [`PUB_OFF!`], and [`pub_off`] macros
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
/// [`GetPubFieldOffset`]: ./trait.GetPubFieldOffset.html
///
/// [`OFF!`]: ../macro.OFF.html
/// [`off`]: ../macro.off.html
/// [`PUB_OFF!`]: ../macro.PUB_OFF.html
/// [`pub_off`]: ../macro.pub_off.html
///
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
    /// - [`IsPublic`]: When the field is `pub`.
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
/// # Example
///
/// Defining a generic method for all types that have `a`, and `c` fields
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     get_field_offset::FieldType,
///     privacy::{IsPublic, IsPrivate},
///     tstr::TS,
///     pub_off,
///     Aligned, GetPubFieldOffset, ROExtAcc,
/// };
///
/// use std::fmt::Debug;
///
/// print_a_c(&ReprC{a: 10, b: 20, c: 30, d: 40 });
///
/// fn print_a_c<T>(this: &T)
/// where
///     T: GetPubFieldOffset<TS!(a), Alignment = Aligned>,
///     T: GetPubFieldOffset<TS!(b), Alignment = Aligned>,
///     FieldType<T, TS!(a)>: Debug,
///     FieldType<T, TS!(b)>: Debug,
/// {
///     println!("{:?}", this.f_get(pub_off!(a)));
///     println!("{:?}", this.f_get(pub_off!(b)));
///
/// #   use repr_offset::get_field_offset::FieldAlignment;
/// #   let _: FieldAlignment<T, TS!(a)> = Aligned;
/// #   let _: FieldAlignment<T, TS!(b)> = Aligned;
/// }
///
/// ```
///
/// [`GetFieldOffset`]: ./trait.GetFieldOffset.html
pub trait GetPubFieldOffset<FN>: GetFieldOffset<FN, Privacy = IsPublic> {
    /// The offset of the field.
    const OFFSET: FieldOffset<Self, Self::Type, Self::Alignment> =
        <Self as GetFieldOffset<FN>>::OFFSET_WITH_VIS.to_field_offset();
}

impl<FN, Ty> GetPubFieldOffset<FN> for Ty where Ty: GetFieldOffset<FN, Privacy = IsPublic> {}

//////////////////////////////////////////////////////////////////////////////////

// Hack to assert that a type implements GetPubFieldOffset,
// while getting the associated types from GetFieldOffset.
use alias_helpers::AssertImplsGPFO;
mod alias_helpers {
    use super::GetFieldOffset;
    use crate::privacy::IsPublic;

    pub type AssertImplsGPFO<This, FN> = <This as AssertPublicField<FN>>::This;

    pub trait AssertPublicField<FN>: GetFieldOffset<FN, Privacy = IsPublic> {
        type This: GetFieldOffset<FN, Privacy = IsPublic>;
    }

    impl<This, FN> AssertPublicField<FN> for This
    where
        This: GetFieldOffset<FN, Privacy = IsPublic>,
    {
        type This = This;
    }
}

//////////////////////////////////////////////////////////////////////////////////

/// Gets the type of a public field in the `GetPubFieldOffset<FN>` impl for `This`.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprC,
///     tstr::TS,
///     FieldType,
/// };
///
/// type This = ReprC<u8, &'static str, Option<usize>, bool>;
///
/// let _: FieldType<This, TS!(a)> = 3_u8;
/// let _: FieldType<This, TS!(b)> = "hello";
/// let _: FieldType<This, TS!(c)> = Some(5_usize);
/// let _: FieldType<This, TS!(d)> = false;
///
/// ```
pub type FieldType<This, FN> = <AssertImplsGPFO<This, FN> as GetFieldOffset<FN>>::Type;

/// Gets the alignment of a public field in the `GetPubFieldOffset<FN>` impl for `This`.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     get_field_offset::FieldAlignment,
///     for_examples::{ReprC, ReprPacked},
///     tstr::TS,
///     Aligned, Unaligned,
/// };
///
/// type Inner = ReprPacked<i16, i32, i64, i128>;
///
/// type This = ReprC<Inner, &'static str, Option<usize>, bool>;
///
/// // Fields directly inside a ReprC are all aligned.
/// let _: FieldAlignment<This, TS!(a)> = Aligned;
/// let _: FieldAlignment<This, TS!(b)> = Aligned;
/// let _: FieldAlignment<This, TS!(c)> = Aligned;
/// let _: FieldAlignment<This, TS!(d)> = Aligned;
///
/// // Fields inside a ReprPacked are all unaligned.
/// let _: FieldAlignment<This, TS!(a, a)> = Unaligned;
/// let _: FieldAlignment<This, TS!(a, b)> = Unaligned;
/// let _: FieldAlignment<This, TS!(a, c)> = Unaligned;
/// let _: FieldAlignment<This, TS!(a, d)> = Unaligned;
///
/// ```
pub type FieldAlignment<This, FN> = <AssertImplsGPFO<This, FN> as GetFieldOffset<FN>>::Alignment;

////////////////////////////////////////////////////////////////////////////////

/// Gets the type of a (potentially) private field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the type of fields in types from external crates.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     get_field_offset::PrivFieldType,
///     tstr::TS,
///     unsafe_struct_field_offsets,
/// };
///
/// use foo::Foo;
///
/// let _: PrivFieldType<Foo, TS!(x)> = 3_u8;
/// let _: PrivFieldType<Foo, TS!(y)> = 5_u16;
/// let _: PrivFieldType<Foo, TS!(z)> = 8_u32;
/// let _: PrivFieldType<Foo, TS!(w)> = 13_u64;
///
/// mod foo {
///     use super::*;
///
///     #[repr(C)]
///     pub struct Foo {
///         x: u8,
///         pub(super) y: u16,
///         pub(crate) z: u32,
///         pub w: u64,
///     }
///
///     repr_offset::unsafe_struct_field_offsets!{
///         alignment = repr_offset::Aligned,
///    
///         impl[] Foo {
///             const OFFSET_X, x: u8;
///             pub(super) const OFFSET_Y, y: u16;
///             pub(crate) const OFFSET_Z, z: u32;
///             pub const OFFSET_W, w: u64;
///         }
///     }
/// }
///
/// ```
///
pub type PrivFieldType<This, FN> = <This as GetFieldOffset<FN>>::Type;

/// Gets the alignment of a (potentially) private field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the alignment of fields in types from external crates.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     for_examples::ReprPacked,
///     get_field_offset::PrivFieldAlignment,
///     tstr::TS,
///     Aligned, Unaligned,
/// };
///
/// # fn main(){
/// // Fields in ReprC are all aligned
/// let _: PrivFieldAlignment<Foo, TS!(x)> = Aligned;
/// let _: PrivFieldAlignment<Foo, TS!(y)> = Aligned;
/// let _: PrivFieldAlignment<Foo, TS!(z)> = Aligned;
/// let _: PrivFieldAlignment<Foo, TS!(w)> = Aligned;
///
/// // Fields in ReprPacked are all unaligned
/// let _: PrivFieldAlignment<Foo, TS!(y, a)> = Unaligned;
/// let _: PrivFieldAlignment<Foo, TS!(y, b)> = Unaligned;
/// let _: PrivFieldAlignment<Foo, TS!(y, c)> = Unaligned;
/// let _: PrivFieldAlignment<Foo, TS!(y, d)> = Unaligned;
/// # }
///
/// mod foo {
///     use super::*;
///     
///     type YField = ReprPacked<&'static str, &'static [u8], char, bool>;
///    
///     #[repr(C)]
///     pub struct Foo {
///         x: u8,
///         pub(super) y: YField,
///         pub(crate) z: u32,
///         pub w: u64,
///     }
///
///     repr_offset::unsafe_struct_field_offsets!{
///         alignment =  Aligned,
///    
///         impl[] Foo {
///             const OFFSET_X, x: u8;
///             pub(super) const OFFSET_Y, y: YField;
///             pub(crate) const OFFSET_Z, z: u32;
///             pub const OFFSET_W, w: u64;
///         }
///     }
/// }
///
/// use foo::Foo;
///
///
/// ```
///
pub type PrivFieldAlignment<This, FN> = <This as GetFieldOffset<FN>>::Alignment;

/// Gets the privacy of a field in the `GetFieldOffset<FN>` impl for `This`.
///
/// # Warning
///
/// Because the field may be private this can break when asking for
/// the privacy of fields in types from external crates.
///
/// # Example
///
/// ```rust
/// use repr_offset::{
///     get_field_offset::FieldPrivacy,
///     privacy::{IsPrivate, IsPublic},
///     tstr::TS,
///     Aligned,
/// };
///
/// let _: FieldPrivacy<Foo, TS!(x)> = IsPrivate;
/// let _: FieldPrivacy<Foo, TS!(y)> = IsPrivate;
/// let _: FieldPrivacy<Foo, TS!(z)> = IsPrivate;
/// let _: FieldPrivacy<Foo, TS!(w)> = IsPublic;
///
/// mod foo {
///     use super::*;
///
///     #[repr(C)]
///     pub struct Foo {
///         x: u8,
///         pub(super) y: u16,
///         pub(crate) z: u32,
///         pub w: u64,
///     }
///
///     repr_offset::unsafe_struct_field_offsets!{
///         alignment = repr_offset::Aligned,
///    
///         impl[] Foo {
///             const OFFSET_X, x: u8;
///             pub(super) const OFFSET_Y, y: u16;
///             pub(crate) const OFFSET_Z, z: u32;
///             pub const OFFSET_W, w: u64;
///         }
///     }
/// }
///
/// use foo::Foo;
///
///
///
/// ```
///
pub type FieldPrivacy<This, FN> = <This as GetFieldOffset<FN>>::Privacy;

//////////////////////////////////////////////////////////////////////////////////

/// A wrapper around a [`FieldOffset`], with a visibility type parameter
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
    /// The `V` type parameter must be:
    /// - [`IsPublic`]: When the field is `pub`.
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
    /// [`FieldOffset::new`]: ../struct.FieldOffset.html#method.new
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
    /// - [`IsPublic`]: When the field is `pub`.
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
    /// [`FieldOffset::new`]: ../struct.FieldOffset.html#method.new
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
    ///
    /// [`FieldOffset`]: ../struct.FieldOffset.html
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
    use super::GetFieldOffset;
    use crate::FieldOffset;

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
