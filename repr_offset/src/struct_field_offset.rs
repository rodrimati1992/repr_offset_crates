// All the uses of usize as isize are for struct offsets,
// which as far as I am aware are all smaller than isize::MAX
#![allow(clippy::ptr_offset_with_cast)]

use crate::{
    alignment::{Aligned, Alignment, CombinePacking, CombinePackingOut, Unaligned},
    offset_calc::GetNextFieldOffset,
    utils::Mem,
};

use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem::MaybeUninit,
    ops::Add,
};

/// Represents the offset of a (potentially nested) field inside a type.
///
/// # Type parameters
///
/// The type parameters are:
///
/// - `S`(for `struct`): the struct that contains the field that this is an offset for.
///
/// - `F`(for field): the type of the field this is an offset for.
///
/// - `A`(for alignment):
/// Is [`Aligned`] if this offset is for [an aligned field](#alignment-guidelines)
/// within the `S` struct,
/// [`Unaligned`] if it is for [an unaligned field](#alignment-guidelines).
/// This changes which methods are available,and the implementation of many of them.
///
/// # Safety
///
/// ### Alignment
///
/// All the unsafe methods for `FieldOffset<_, _, Aligned>`
/// that move/copy the field require that
/// the passed in pointers are aligned,
/// while the ones for `FieldOffset<_, _, Unaligned>` do not.
///
/// Because passing unaligned pointers to `FieldOffset<_, _, Aligned>` methods
/// causes undefined behavior,
/// you must be careful when accessing a nested field in `#[repr(C, packed)]` structs.
///
/// For an example of how to correctly access nested fields inside of
/// `#[repr(C, packed)]` structs [look here](#nested-field-in-packed).
///
/// <span id="alignment-guidelines"></span>
/// # Field alignment guidelines
///
/// A non-nested field is:
///
/// - Aligned: if the type that contains the field is
/// `#[repr(C)]`/`#[repr(C, align(...))]`/`#[repr(transparent)]`.
///
/// - Unaligned: if the type that contains the field is `#[repr(C, packed(....))]`,
/// and the packing is smaller than the alignment of the field type.<br>
/// Note that type alignment can vary across platforms,
/// so `FieldOffset<S, F, Unaligned>`(as opposed to `FieldOffset<S, F, Aligned>`)
/// is safest when `S` is a `#[repr(C, packed)]` type.
///
/// A nested field is unaligned if any field in the chain of field accesses to the
/// nested field (ie: `foo` and `bar` and `baz` in `foo.bar.baz`)
/// is unaligned according to the rules for non-nested fields described in this section.
///
///
/// # Examples
///
/// ### No Macros
///
/// This example demonstrates how you can construct `FieldOffset` without macros.
///
/// You can use the [`ReprOffset`] derive macro or [`unsafe_struct_field_offsets`] macro
/// to construct the constants more conveniently (and in a less error-prone way).
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{Aligned, FieldOffset};
///
/// use std::mem;
///
///
/// fn main(){
///     let mut foo = Foo{ first: 3u16, second: 5, third: None };
///
///     *Foo::OFFSET_FIRST.get_mut(&mut foo) = 13;
///     *Foo::OFFSET_SECOND.get_mut(&mut foo) = 21;
///     *Foo::OFFSET_THIRD.get_mut(&mut foo) = Some(34);
///
///     assert_eq!( foo, Foo{ first: 13, second: 21, third: Some(34) } );
/// }
///
///
/// #[repr(C)]
/// #[derive(Debug,PartialEq)]
/// struct Foo<T>{
///     first: T,
///     second: u32,
///     third: Option<T>,
/// }
///
/// impl<T> Foo<T>{
///     const OFFSET_FIRST: FieldOffset<Self, T, Aligned> = unsafe{ FieldOffset::new(0) };
///
///     const OFFSET_SECOND: FieldOffset<Self, u32, Aligned> = unsafe{
///         Self::OFFSET_FIRST.next_field_offset()
///     };
///
///     const OFFSET_THIRD: FieldOffset<Self, Option<T>, Aligned> = unsafe{
///         Self::OFFSET_SECOND.next_field_offset()
///     };
/// }
///
/// ```
///
/// <span id="nested-field-in-packed"></span>
/// ### Accessing Nested Fields
///
/// This example demonstrates how to access nested fields in a `#[repr(C, packed)]` struct.
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
#[cfg_attr(feature = "derive", doc = "use repr_offset::ReprOffset;")]
#[cfg_attr(not(feature = "derive"), doc = "use repr_offset_derive::ReprOffset;")]
/// use repr_offset::{Aligned, FieldOffset, Unaligned};
///
/// #[repr(C, packed)]
/// #[derive(ReprOffset)]
/// struct Pack{
///     x: u8,
///     y: NestedC,
/// }
///
/// #[repr(C)]
/// #[derive(ReprOffset)]
/// struct NestedC{
///     name: &'static str,
///     years: usize,
/// }
///
/// const OFFY: FieldOffset<Pack, NestedC, Unaligned> = Pack::OFFSET_Y;
///
/// let _: FieldOffset<NestedC, &'static str, Aligned> = NestedC::OFFSET_NAME;
/// let _: FieldOffset<NestedC, usize, Aligned> = NestedC::OFFSET_YEARS;
///
/// // As you can see `FieldOffset::add` combines two offsets,
/// // allowing you to access a nested field with a single `FieldOffset`.
/// //
/// // These `FieldOffset`s have an `Unaligned` type argument because
/// // OFFY is a `FieldOffset<_, _, Unaligned>`.
/// const OFF_NAME: FieldOffset<Pack, &'static str, Unaligned> = OFFY.add(NestedC::OFFSET_NAME);
/// const OFF_YEARS: FieldOffset<Pack, usize, Unaligned> = OFFY.add(NestedC::OFFSET_YEARS);
///
/// let this = Pack{
///     x: 0,
///     y: NestedC{ name: "John", years: 13 },
/// };
///
/// assert_eq!(OFF_NAME.get_copy(&this), "John" );
/// assert_eq!(OFF_YEARS.get_copy(&this), 13 );
///
/// unsafe{
///     let nested_ptr: *const NestedC = OFFY.get_ptr(&this);
///
///     // This code is undefined behavior,
///     // because `NestedC`'s offsets require the passed in pointer to be aligned.
///     //
///     // assert_eq!(NestedC::OFFSET_NAME.read(nested_ptr), "John" );
///     // assert_eq!(NestedC::OFFSET_YEARS.read(nested_ptr), 13 );
///
///     // This is fine though,because the offsets were turned into
///     // `FieldOffset<_, _, Unaligned>` with `.to_unaligned()`.
///     assert_eq!( NestedC::OFFSET_NAME.to_unaligned().read(nested_ptr), "John" );
///     assert_eq!( NestedC::OFFSET_YEARS.to_unaligned().read(nested_ptr), 13 );
///
/// }
/// ```
///
/// [`Aligned`]: ./struct.Aligned.html
/// [`Unaligned`]: ./struct.Unaligned.html
///
/// [`ReprOffset`]: ./docs/repr_offset_macro/index.html
/// [`unsafe_struct_field_offsets`]: ./macro.unsafe_struct_field_offsets.html
///
#[repr(transparent)]
pub struct FieldOffset<S, F, A> {
    offset: usize,
    _marker: PhantomData<DummyType<(S, F, A)>>,
}

// Workaround for `PhantomData<fn()->T>` not being constructible in const contexts
struct DummyType<T>(fn() -> T);

impl_cmp_traits_for_offset! {
    impl[S, F, A] FieldOffset<S, F, A>
}

impl<S, F, A> Debug for FieldOffset<S, F, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FieldOffset")
            .field("offset", &self.offset)
            .finish()
    }
}

impl<S, F, A> Copy for FieldOffset<S, F, A> {}

impl<S, F, A> Clone for FieldOffset<S, F, A> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `raw_get`
macro_rules! get_ptr_method {
    ($self:ident, $base:expr, $F:ty) => {
        ($base as *const _ as *const u8).offset($self.offset as isize) as *const $F
    };
}

// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `raw_get_mut`
macro_rules! get_mut_ptr_method {
    ($self:ident, $base:expr, $F:ty) => {
        ($base as *mut _ as *mut u8).offset($self.offset as isize) as *mut $F
    };
}

impl<S, F, A> FieldOffset<S, F, A> {
    /// Constructs this `FieldOffset` from the offset of the field.
    ///
    /// # Safety
    ///
    /// Callers must ensure all of these:
    ///
    /// - `S` must be a `#[repr(C)]` or `#[repr(transparent)]` struct
    /// (optionally with `align` or `packed` attributes).
    ///
    /// - `offset` must be the byte offset of a field of type `F` inside the struct `S`.
    ///
    /// - The `A` type parameter must be [`Unaligned`]
    /// if the field [is unaligned](#alignment-guidelines),
    /// or [`Aligned`] if [it is aligned](#alignment-guidelines).
    ///
    /// # Example
    ///
    /// Constructing the `FieldOffset`s of a packed struct.
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{Aligned, FieldOffset, Unaligned};
    ///
    /// let this = Packed{ x: 3, y: 5, z: "huh" };
    ///
    /// assert_eq!( OFFSET_X.get_copy(&this), 3 );
    /// assert_eq!( OFFSET_Y.get_copy(&this), 5 );
    /// assert_eq!( OFFSET_Z.get_copy(&this), "huh" );
    ///
    /// #[repr(C, packed)]
    /// struct Packed{
    ///     x: u8,
    ///     y: u32,
    ///     z: &'static str,
    /// }
    ///
    /// // `u8` is always aligned.
    /// const OFFSET_X: FieldOffset<Packed, u8, Aligned> = unsafe{
    ///     FieldOffset::new(0)
    /// };
    /// const OFFSET_Y: FieldOffset<Packed, u32, Unaligned> = unsafe{
    ///     OFFSET_X.next_field_offset()
    /// };
    /// const OFFSET_Z: FieldOffset<Packed, &'static str, Unaligned> = unsafe{
    ///     OFFSET_Y.next_field_offset()
    /// };
    ///
    /// ```
    /// [`Aligned`]: ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
    #[inline(always)]
    pub const unsafe fn new(offset: usize) -> Self {
        Self {
            offset,
            _marker: PhantomData,
        }
    }

    // This must be kept private
    #[inline(always)]
    const fn priv_new(offset: usize) -> Self {
        Self {
            offset,
            _marker: PhantomData,
        }
    }

    /// Constructs a `FieldOffset` by calculating the offset of the next field.
    ///
    /// # Safety
    ///
    /// Callers must ensure that:
    ///
    /// - `Next` is the type of the field after the one that this is an offset for.
    ///
    /// - `NextA` must be [`Unaligned`] if the field [is unaligned](#alignment-guidelines),
    /// or [`Aligned`] if [it is aligned](#alignment-guidelines).
    ///
    /// # Example
    ///
    /// Constructing the `FieldOffset`s of a `#[repr(C, align(16))]` struct.
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{Aligned, FieldOffset};
    ///
    /// let this = ReprAligned{ foo: true, bar: Some('8'), baz: 55 };
    ///
    /// assert_eq!( OFFSET_FOO.get_copy(&this), true );
    /// assert_eq!( OFFSET_BAR.get_copy(&this), Some('8') );
    /// assert_eq!( OFFSET_BAZ.get_copy(&this), 55 );
    ///
    ///
    /// #[repr(C, align(16))]
    /// struct ReprAligned{
    ///     foo: bool,
    ///     bar: Option<char>,
    ///     baz: u64,
    /// }
    ///
    /// const OFFSET_FOO: FieldOffset<ReprAligned, bool, Aligned> = unsafe{
    ///     FieldOffset::new(0)
    /// };
    /// const OFFSET_BAR: FieldOffset<ReprAligned, Option<char>, Aligned> = unsafe{
    ///     OFFSET_FOO.next_field_offset()
    /// };
    /// const OFFSET_BAZ: FieldOffset<ReprAligned, u64, Aligned> = unsafe{
    ///     OFFSET_BAR.next_field_offset()
    /// };
    ///
    /// ```
    ///
    /// [`Aligned`]: ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
    pub const unsafe fn next_field_offset<Next, NextA>(self) -> FieldOffset<S, Next, NextA> {
        let offset = GetNextFieldOffset {
            previous_offset: self.offset,
            previous_size: Mem::<F>::SIZE,
            container_alignment: Mem::<S>::ALIGN,
            next_alignment: Mem::<Next>::ALIGN,
        }
        .call();

        FieldOffset {
            offset,
            _marker: PhantomData,
        }
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Combines this `FieldOffset` with another one, to access a nested field.
    ///
    /// Note that the resulting `FieldOffset` has the
    /// alignment type parameter (the third one) of `other`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{Aligned, FieldOffset, Unaligned};
    /// use repr_offset::for_examples::{ReprC, ReprPacked};
    ///
    /// type This = ReprC<char, ReprC<u8, u16>, ReprPacked<u32, u64>>;
    ///
    /// let this: This = ReprC {
    ///     a: '3',
    ///     b: ReprC{ a: 5u8, b: 8u16, c: (), d: () },
    ///     c: ReprPacked{ a: 13u32, b: 21u64, c: (), d: () },
    ///     d: (),
    /// };
    ///
    /// assert_eq!( OFFSET_B_A.get_copy(&this), 5 );
    /// assert_eq!( OFFSET_C_A.get_copy(&this), 13 );
    ///
    /// // This is the FieldOffset of the `.b.a` nested field.
    /// const OFFSET_B_A: FieldOffset<This, u8, Aligned> =
    ///     ReprC::OFFSET_B.add(ReprC::OFFSET_A);
    ///
    /// // This is the FieldOffset of the `.c.a` nested field.
    /// //
    /// // The alignment type parameter of the combined FieldOffset is`Unaligned` if
    /// // either FieldOffset has `Unaligned` as the `A` type parameter.
    /// const OFFSET_C_A: FieldOffset<This, u32, Unaligned> =
    ///     ReprC::OFFSET_C.add(ReprPacked::OFFSET_A);
    ///
    /// ```
    ///
    #[inline(always)]
    pub const fn add<F2, A2>(self, other: FieldOffset<F, F2, A2>) -> FieldOffset<S, F2, A2> {
        FieldOffset::priv_new(self.offset + other.offset)
    }
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Combines this `FieldOffset` with another one, to access a nested field.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{FieldOffset, Unaligned};
    /// use repr_offset::for_examples::{ReprC, ReprPacked};
    ///
    /// type This = ReprPacked<char, ReprC<u8, u16>, ReprPacked<u32, u64>>;
    ///
    /// let this: This = ReprPacked {
    ///     a: '3',
    ///     b: ReprC{ a: 34u8, b: 55u16, c: (), d: () },
    ///     c: ReprPacked{ a: 89u32, b: 144u64, c: (), d: () },
    ///     d: (),
    /// };
    ///
    /// assert_eq!( OFFSET_B_A.get_copy(&this), 34 );
    /// assert_eq!( OFFSET_C_A.get_copy(&this), 89 );
    ///
    /// // This is the FieldOffset of the `.b.a` nested field.
    /// const OFFSET_B_A: FieldOffset<This, u8, Unaligned> =
    ///     ReprPacked::OFFSET_B.add(ReprC::OFFSET_A);
    ///
    /// // This is the FieldOffset of the `.c.a` nested field.
    /// const OFFSET_C_A: FieldOffset<This, u32, Unaligned> =
    ///     ReprPacked::OFFSET_C.add(ReprPacked::OFFSET_A);
    ///
    /// ```
    ///
    #[inline(always)]
    pub const fn add<F2, A2>(self, other: FieldOffset<F, F2, A2>) -> FieldOffset<S, F2, Unaligned> {
        FieldOffset::priv_new(self.offset + other.offset)
    }
}

/// Equivalent to the inherent `FieldOffset::add` method,
/// that one can be ran at compile-time(this one can't).
///
/// # Example
///
/// ```rust
/// # #![deny(safe_packed_borrows)]
/// use repr_offset::{Aligned, FieldOffset, Unaligned};
/// use repr_offset::for_examples::{ReprC, ReprPacked};
///
/// type This = ReprC<char, ReprC<u8, u16>, ReprPacked<u32, u64>>;
///
/// let this: This = ReprC {
///     a: '3',
///     b: ReprC{ a: 5u8, b: 8u16, c: (), d: () },
///     c: ReprPacked{ a: 13u32, b: 21u64, c: (), d: () },
///     d: (),
/// };
///
/// // This is the FieldOffset of the `.b.a` nested field.
/// let offset_b_b = ReprC::OFFSET_B + ReprC::OFFSET_B;
///
/// // This is the FieldOffset of the `.c.a` nested field.
/// let offset_c_b = ReprC::OFFSET_C + ReprPacked::OFFSET_B;
///
/// assert_eq!( offset_b_b.get_copy(&this), 8 );
/// assert_eq!( offset_c_b.get_copy(&this), 21 );
///
/// ```
///
impl<S, F, A, F2, A2> Add<FieldOffset<F, F2, A2>> for FieldOffset<S, F, A>
where
    A: CombinePacking<A2>,
    A2: Alignment,
{
    type Output = FieldOffset<S, F2, CombinePackingOut<A, A2>>;

    #[inline(always)]
    fn add(self, other: FieldOffset<F, F2, A2>) -> Self::Output {
        FieldOffset::priv_new(self.offset + other.offset)
    }
}

impl<S, F, A> FieldOffset<S, F, A> {
    /// The offset (in bytes) of the `F` field in the `S` struct.
    ///
    /// # Example
    ///
    /// This example demonstrates this method with a `#[repr(C, packed)]` struct.
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// type Normal = ReprPacked<u8, u16, u32, u64>;
    /// type Reversed = ReprPacked<u64, u32, u16, u8>;
    ///
    /// assert_eq!( Normal::OFFSET_A.offset(), 0 );
    /// assert_eq!( Normal::OFFSET_B.offset(), 1 );
    /// assert_eq!( Normal::OFFSET_C.offset(), 3 );
    /// assert_eq!( Normal::OFFSET_D.offset(), 7 );
    ///
    /// assert_eq!( Reversed::OFFSET_A.offset(), 0 );
    /// assert_eq!( Reversed::OFFSET_B.offset(), 8 );
    /// assert_eq!( Reversed::OFFSET_C.offset(), 12 );
    /// assert_eq!( Reversed::OFFSET_D.offset(), 14 );
    ///
    ///
    /// ```
    #[inline(always)]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Changes the `S` type parameter, most useful for `#[repr(transparent)]` wrappers.
    ///
    /// # Safety
    ///
    /// Callers must ensure that there is a field of type `F` at the same offset
    /// inside the `S2` type,
    /// and is at least as public as this `FieldOffset`.
    ///
    /// If the `A` type parameter is [`Aligned`],
    /// then the field [must be aligned](#alignment-guidelines)
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = Wrapper(ReprC{
    ///     a: false,
    ///     b: 3u8,
    ///     c: Some('5'),
    ///     d: [8u32, 13u32],
    /// });
    ///
    /// assert_eq!( cast_offset(ReprC::OFFSET_A).get(&this), &false );
    /// assert_eq!( cast_offset(ReprC::OFFSET_B).get(&this), &3u8 );
    /// assert_eq!( cast_offset(ReprC::OFFSET_C).get(&this), &Some('5') );
    /// assert_eq!( cast_offset(ReprC::OFFSET_D).get(&this), &[8u32, 13u32] );
    ///
    ///
    /// #[repr(transparent)]
    /// pub struct Wrapper<T>(pub T);
    ///
    /// pub const fn cast_offset<T,F,A>(offset: FieldOffset<T,F,A>) -> FieldOffset<Wrapper<T>,F,A>{
    ///     // safety: This case is safe because this is a
    ///     // `#[repr(transparent)]` wrapper around `T`
    ///     // where `T` is a public field in the wrapper
    ///     unsafe{ offset.cast_struct() }
    /// }
    ///
    ///
    ///
    /// ```
    ///
    /// [`Aligned`]: ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
    #[inline(always)]
    pub const unsafe fn cast_struct<S2>(self) -> FieldOffset<S2, F, A> {
        FieldOffset::new(self.offset)
    }

    /// Changes the `F` type parameter.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the `F2` type is compatible with the `F` type,
    /// including size,alignment, and internal layout.
    ///
    /// If the `F` type encodes an invariant,
    /// then callers must ensure that if the field is used as the `F` type
    /// (including the destructor for the type)
    /// that the invariants for that type must be upheld.
    ///
    /// The same applies if the field is used as the `F2` type
    /// (if the returned FieldOffset isn't used,then it would not be used as the `F2` type)
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    ///
    /// use repr_offset::{Aligned, FieldOffset};
    /// use repr_offset::for_examples::ReprC;
    ///
    /// type This = ReprC<u8, u64, (), ()>;
    ///
    /// let this: This = ReprC{ a: 3, b: 5, c: (), d: () };
    ///
    /// unsafe{
    ///     assert_eq!( This::OFFSET_A.cast_field::<i8>().get(&this), &3i8 );
    ///     assert_eq!( This::OFFSET_B.cast_field::<i64>().get(&this), &5i64 );
    /// }
    ///
    /// ```
    /// [safe and valid]:
    /// https://rust-lang.github.io/unsafe-code-guidelines/glossary.html#validity-and-safety-invariant
    #[inline(always)]
    pub const unsafe fn cast_field<F2>(self) -> FieldOffset<S, F2, A> {
        FieldOffset::new(self.offset)
    }

    /// Changes this `FieldOffset` to be for a (potentially) unaligned field.
    ///
    /// This is useful if you want to get a nested field from an unaligned pointer to a
    /// `#[repr(C)]`/`#[repr(C,align())]` struct.
    ///
    /// # Example
    ///
    /// This example demonstrates how you can copy a field
    /// from an unaligned pointer to a `#[repr(C)]` struct.
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::{ReprC, ReprPacked};
    ///
    /// type Inner = ReprC<usize, &'static str>;
    /// type Outer = ReprPacked<u8, Inner>;
    ///
    /// let inner = ReprC { a: 3, b: "5", c: (), d: () };
    /// let outer: Outer = ReprPacked{ a: 21, b: inner, c: (), d: () };
    ///
    /// let inner_ptr: *const Inner = Outer::OFFSET_B.get_ptr(&outer);
    /// unsafe{
    ///     assert_eq!( Inner::OFFSET_A.to_unaligned().read_copy(inner_ptr), 3 );
    ///     assert_eq!( Inner::OFFSET_B.to_unaligned().read_copy(inner_ptr), "5" );
    ///
    ///     // This is undefined behavior,
    ///     // because ReprC's FieldOFfsets require the pointer to be aligned.
    ///     //
    ///     // assert_eq!( Inner::OFFSET_A.read_copy(inner_ptr), 3 );
    ///     // assert_eq!( Inner::OFFSET_B.read_copy(inner_ptr), "5" );
    /// }
    ///
    /// ```
    ///
    #[inline(always)]
    pub const fn to_unaligned(self) -> FieldOffset<S, F, Unaligned> {
        FieldOffset {
            offset: self.offset,
            _marker: PhantomData,
        }
    }

    /// Changes this `FieldOffset` to be for an aligned field.
    ///
    /// # Safety
    ///
    /// Callers must ensure that [the field is aligned](#alignment-guidelines)
    /// within the `S` type.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::{Aligned, FieldOffset, Unaligned};
    ///
    /// // ReprPacked2 is aligned to 2 bytes.
    /// use repr_offset::for_examples::ReprPacked2;
    ///
    /// type This = ReprPacked2<u8, u16, (), ()>;
    ///
    /// let _: FieldOffset<This, u8, Unaligned> = This::OFFSET_A;
    /// let _: FieldOffset<This, u16, Unaligned> = This::OFFSET_B;
    ///
    /// let this: This = ReprPacked2{ a: 89, b: 144, c: (), d: () };
    ///
    /// unsafe{
    ///     assert_eq!( This::OFFSET_A.to_aligned().get(&this), &89 );
    ///     assert_eq!( This::OFFSET_B.to_aligned().get(&this), &144 );
    /// }
    /// ```
    #[inline(always)]
    pub const unsafe fn to_aligned(self) -> FieldOffset<S, F, Aligned> {
        FieldOffset::new(self.offset)
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Gets a reference to the field that this is an offset for.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: '@', b: 21u8, c: (), d: () };
    ///
    /// assert_eq!( ReprC::OFFSET_A.get(&this), &'@' );
    /// assert_eq!( ReprC::OFFSET_B.get(&this), &21u8 );
    ///
    /// ```
    #[inline(always)]
    pub fn get(self, base: &S) -> &F {
        unsafe { &*get_ptr_method!(self, base, F) }
    }

    /// Gets a mutable reference to the field that this is an offset for.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: "what", b: '?', c: (), d: () };
    ///
    /// assert_eq!( ReprC::OFFSET_A.get_mut(&mut this), &mut "what" );
    /// assert_eq!( ReprC::OFFSET_B.get_mut(&mut this), &mut '?' );
    ///
    /// ```
    #[inline(always)]
    pub fn get_mut(self, base: &mut S) -> &mut F {
        unsafe { &mut *get_mut_ptr_method!(self, base, F) }
    }

    /// Copies the aligned field that this is an offset for.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: Some(false), b: [8i32, 13, 21], c: (), d: () };
    ///
    /// assert_eq!( ReprC::OFFSET_A.get_copy(&this), Some(false) );
    /// assert_eq!( ReprC::OFFSET_B.get_copy(&this), [8i32, 13, 21] );
    ///
    /// ```
    ///
    /// This method can't be called for non-Copy fields.
    /// ```compile_fail
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: vec![0, 1, 2, 3], b: (), c: (), d: () };
    ///
    /// let _ = ReprC::OFFSET_A.get_copy(&this);
    /// ```
    #[inline(always)]
    pub fn get_copy(self, base: &S) -> F
    where
        F: Copy,
    {
        unsafe { *get_ptr_method!(self, base, F) }
    }
}

impl<S, F, A> FieldOffset<S, F, A> {
    /// Gets a raw pointer to a field from a reference to the `S` struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr_a = ReprPacked::OFFSET_A.get_ptr(&this);
    /// // A `u8` is always aligned,so a `.read()` is fine.
    /// assert_eq!( unsafe{ ptr_a.read() }, 3u8 );
    ///
    /// let ptr_b = ReprPacked::OFFSET_B.get_ptr(&this);
    /// // ReprPacked has an alignment of 1,
    /// // so this u16 field has to be copied with `.read_unaligned()`.
    /// assert_eq!( unsafe{ ptr_b.read_unaligned() }, 5u16 );
    ///
    /// ```
    #[inline(always)]
    pub fn get_ptr(self, base: &S) -> *const F {
        unsafe { get_ptr_method!(self, base, F) }
    }

    /// Gets a mutable raw pointer to a field from a mutable reference to the `S` struct.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let mut this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr_a = ReprPacked::OFFSET_A.get_mut_ptr(&mut this);
    /// unsafe{
    ///     // A `u8` is always aligned,so a `.read()` is fine.
    ///     assert_eq!( ptr_a.read(), 3u8 );
    ///     ptr_a.write(103);
    ///     assert_eq!( ptr_a.read(), 103 );
    /// }
    ///
    /// let ptr_b = ReprPacked::OFFSET_B.get_mut_ptr(&mut this);
    /// unsafe{
    ///     // ReprPacked has an alignment of 1,
    ///     // so this u16 field has to be read with `.read_unaligned()`.
    ///     assert_eq!( ptr_b.read_unaligned(), 5u16 );
    ///     ptr_b.write_unaligned(105);
    ///     assert_eq!( ptr_b.read_unaligned(), 105 );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub fn get_mut_ptr(self, base: &mut S) -> *mut F {
        unsafe { get_mut_ptr_method!(self, base, F) }
    }

    /// Gets a raw pointer to a field from a pointer to the `S` struct.
    ///
    /// # Safety
    ///
    /// This has the same safety requirements as the [`<*const T>::offset`] method.
    ///
    /// [`<*const T>::offset`]:
    /// https://doc.rust-lang.org/std/primitive.pointer.html#method.offset
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr: *const _ = &this;
    ///
    /// unsafe{
    ///     // A `u8` is always aligned,so a `.read()` is fine.
    ///     assert_eq!( ReprPacked::OFFSET_A.raw_get(ptr).read(), 3u8 );
    ///     
    ///     // ReprPacked has an alignment of 1,
    ///     // so this u16 field has to be copied with `.read_unaligned()`.
    ///     assert_eq!( ReprPacked::OFFSET_B.raw_get(ptr).read_unaligned(), 5u16 );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn raw_get(self, base: *const S) -> *const F {
        get_ptr_method!(self, base, F)
    }

    /// Gets a mutable raw pointer to a field from a pointer to the `S` struct.
    ///
    /// # Safety
    ///
    /// This has the same safety requirements as the [`<*mut T>::offset`] method.
    ///
    /// [`<*mut T>::offset`]:
    /// https://doc.rust-lang.org/std/primitive.pointer.html#method.offset-1
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let mut this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    ///
    /// unsafe{
    ///     let ptr_a = ReprPacked::OFFSET_A.raw_get_mut(ptr);
    ///
    ///     // A `u8` is always aligned,so a `.read()` is fine.
    ///     assert_eq!( ptr_a.read(), 3u8 );
    ///     ptr_a.write(103);
    ///     assert_eq!( ptr_a.read(), 103 );
    ///
    ///
    ///     let ptr_b = ReprPacked::OFFSET_B.raw_get_mut(ptr);
    ///
    ///     // ReprPacked has an alignment of 1,
    ///     // so this u16 field has to be read with `.read_unaligned()`.
    ///     assert_eq!( ptr_b.read_unaligned(), 5u16 );
    ///     ptr_b.write_unaligned(105);
    ///     assert_eq!( ptr_b.read_unaligned(), 105 );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn raw_get_mut(self, base: *mut S) -> *mut F {
        get_mut_ptr_method!(self, base, F)
    }

    /// Gets a raw pointer to a field from a pointer to the `S` struct.
    ///
    /// # Safety
    ///
    /// While calling this method is not by itself unsafe,
    /// using the pointer returned by this method has the same safety requirements
    /// as the [`<*const T>::wrapping_offset`] method.
    ///
    /// [`<*const T>::wrapping_offset`]:
    /// https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr_a = ReprPacked::OFFSET_A.wrapping_raw_get(&this);
    /// // A `u8` is always aligned,so a `.read()` is fine.
    /// assert_eq!( unsafe{ ptr_a.read() }, 3u8 );
    ///
    /// let ptr_b = ReprPacked::OFFSET_B.wrapping_raw_get(&this);
    /// // ReprPacked has an alignment of 1,
    /// // so this u16 field has to be copied with `.read_unaligned()`.
    /// assert_eq!( unsafe{ ptr_b.read_unaligned() }, 5u16 );
    ///
    /// ```
    #[inline(always)]
    pub fn wrapping_raw_get(self, base: *const S) -> *const F {
        (base as *const u8).wrapping_offset(self.offset as isize) as *const F
    }

    /// Gets a mutable raw pointer to a field from a pointer to the `S` struct.
    ///
    /// # Safety
    ///
    /// While calling this method is not by itself unsafe,
    /// using the pointer returned by this method has the same safety requirements
    /// as the [`<*mut T>::wrapping_offset`] method.
    ///
    /// [`<*mut T>::wrapping_offset`]:
    /// https://doc.rust-lang.org/std/primitive.pointer.html#method.wrapping_offset-1
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::FieldOffset;
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let mut this = ReprPacked{ a: 3u8, b: 5u16, c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    ///
    /// let ptr_a = ReprPacked::OFFSET_A.wrapping_raw_get_mut(ptr);
    /// unsafe{
    ///
    ///     // A `u8` is always aligned,so a `.read()` is fine.
    ///     assert_eq!( ptr_a.read(), 3u8 );
    ///     ptr_a.write(103);
    ///     assert_eq!( ptr_a.read(), 103 );
    /// }
    ///
    /// let ptr_b = ReprPacked::OFFSET_B.wrapping_raw_get_mut(ptr);
    /// unsafe{
    ///
    ///     // ReprPacked has an alignment of 1,
    ///     // so this u16 field has to be read with `.read_unaligned()`.
    ///     assert_eq!( ptr_b.read_unaligned(), 5u16 );
    ///     ptr_b.write_unaligned(105);
    ///     assert_eq!( ptr_b.read_unaligned(), 105 );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub fn wrapping_raw_get_mut(self, base: *mut S) -> *mut F {
        (base as *mut u8).wrapping_offset(self.offset as isize) as *mut F
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Copies the aligned field that this is an offset for.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read`](https://doc.rust-lang.org/std/ptr/fn.read.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: 10u8, b: "20", c: (), d: () };
    ///
    /// let ptr: *const _ = &this;
    /// unsafe{
    ///     assert_eq!( ReprC::OFFSET_A.read_copy(ptr), 10u8 );
    ///     assert_eq!( ReprC::OFFSET_B.read_copy(ptr), "20" );
    /// }
    /// ```
    ///
    /// This method can't be called for non-Copy fields.
    /// ```compile_fail
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: vec![0, 1, 2, 3], b: (), c: (), d: () };
    /// unsafe{
    ///     let _ = ReprC::OFFSET_A.read_copy(&this);
    /// }
    /// ```
    ///
    #[inline(always)]
    pub unsafe fn read_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        *get_ptr_method!(self, base, F)
    }

    /// Reads the value from the field in `source` without moving it.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read`](https://doc.rust-lang.org/std/ptr/fn.read.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// use std::mem::ManuallyDrop;
    ///
    /// let this = ManuallyDrop::new(ReprC{
    ///     a: vec![0, 1, 2],
    ///     b: "20".to_string(),
    ///     c: (),
    ///     d: (),
    /// });
    ///
    /// let ptr: *const _ = &*this;
    /// unsafe{
    ///     assert_eq!( ReprC::OFFSET_A.read(ptr), vec![0, 1, 2] );
    ///     assert_eq!( ReprC::OFFSET_B.read(ptr), "20".to_string() );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn read(self, source: *const S) -> F {
        get_ptr_method!(self, source, F).read()
    }

    /// Writes `value` ìnto the field in `destination` without dropping the old value of the field.
    ///
    /// This allows uninitialized fields to be initialized,since doing
    /// `*OFFSET_FOO.raw_get_mut(ptr) = value;` would drop uninitialized memory.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: 10u8, b: "20", c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    /// unsafe{
    ///     ReprC::OFFSET_A.write(ptr, 13u8);
    ///     ReprC::OFFSET_B.write(ptr, "21");
    /// }
    /// assert_eq!( this.a, 13u8 );
    /// assert_eq!( this.b, "21" );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn write(self, destination: *mut S, value: F) {
        get_mut_ptr_method!(self, destination, F).write(value)
    }

    /// Copies the field in `source` into `destination`.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::copy`](https://doc.rust-lang.org/std/ptr/fn.copy.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: 10u8, b: "20", c: (), d: () };
    /// let mut other = ReprC{ a: 0u8, b: "", c: (), d: () };
    ///
    /// let this_ptr: *const _ = &this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprC::OFFSET_A.copy(this_ptr, other_ptr);
    ///     ReprC::OFFSET_B.copy(this_ptr, other_ptr);
    /// }
    /// assert_eq!( this.a, 10u8 );
    /// assert_eq!( this.b, "20" );
    ///
    /// assert_eq!( other.a, 10u8 );
    /// assert_eq!( other.b, "20" );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn copy(self, source: *const S, destination: *mut S) {
        core::ptr::copy(
            get_ptr_method!(self, source, F),
            get_mut_ptr_method!(self, destination, F),
            1,
        );
    }

    /// Copies the field in `source` into `destination`,
    /// `source` and `destination` must not overlap.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::copy_nonoverlapping`
    /// ](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let this = ReprC{ a: '#', b: 81, c: (), d: () };
    /// let mut other = ReprC{ a: '_', b: 0, c: (), d: () };
    ///
    /// let this_ptr: *const _ = &this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprC::OFFSET_A.copy_nonoverlapping(this_ptr, other_ptr);
    ///     ReprC::OFFSET_B.copy_nonoverlapping(this_ptr, other_ptr);
    /// }
    /// assert_eq!( this.a, '#' );
    /// assert_eq!( this.b, 81 );
    ///
    /// assert_eq!( other.a, '#' );
    /// assert_eq!( other.b, 81 );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn copy_nonoverlapping(self, source: *const S, destination: *mut S) {
        core::ptr::copy_nonoverlapping(
            get_ptr_method!(self, source, F),
            get_mut_ptr_method!(self, destination, F),
            1,
        );
    }

    /// Replaces the value of a field in `destination` with `value`,
    /// returning the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::replace`](https://doc.rust-lang.org/std/ptr/fn.replace.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: [0u8, 1], b: false, c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    /// unsafe{
    ///     assert_eq!( ReprC::OFFSET_A.replace(ptr, [2, 3]), [0u8, 1] );
    ///     assert_eq!( ReprC::OFFSET_B.replace(ptr, true), false );
    /// }
    ///
    /// assert_eq!( this.a, [2u8, 3] );
    /// assert_eq!( this.b, true );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn replace(self, destination: *mut S, value: F) -> F {
        core::ptr::replace(get_mut_ptr_method!(self, destination, F), value)
    }

    /// Replaces the value of a field in `destination` with `value`,
    /// returning the old value of the field.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: [0u8, 1], b: false, c: (), d: () };
    ///
    /// assert_eq!( ReprC::OFFSET_A.replace_mut(&mut this, [2, 3]), [0u8, 1] );
    /// assert_eq!( ReprC::OFFSET_B.replace_mut(&mut this, true), false );
    ///
    /// assert_eq!( this.a, [2u8, 3] );
    /// assert_eq!( this.b, true );
    ///
    /// ```
    #[inline(always)]
    pub fn replace_mut(self, destination: &mut S, value: F) -> F {
        unsafe { core::mem::replace(&mut *get_mut_ptr_method!(self, destination, F), value) }
    }

    /// Swaps the values of a field between the `left` and `right` pointers.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::swap`](https://doc.rust-lang.org/std/ptr/fn.swap.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: '=', b: 64u16, c: (), d: () };
    /// let mut other = ReprC{ a: '!', b: 255u16, c: (), d: () };
    ///
    /// let this_ptr: *mut _ = &mut this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprC::OFFSET_A.swap(this_ptr, other_ptr);
    ///     ReprC::OFFSET_B.swap(this_ptr, other_ptr);
    /// }
    /// assert_eq!( this.a, '!' );
    /// assert_eq!( this.b, 255 );
    ///
    /// assert_eq!( other.a, '=' );
    /// assert_eq!( other.b, 64 );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn swap(self, left: *mut S, right: *mut S) {
        core::ptr::swap::<F>(
            get_mut_ptr_method!(self, left, F),
            get_mut_ptr_method!(self, right, F),
        )
    }

    /// Swaps the values of a field between the `left` and `right` non-overlapping pointers.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::swap_nonoverlapping`
    /// ](https://doc.rust-lang.org/std/ptr/fn.swap_nonoverlapping.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: [false, true], b: &27u32, c: (), d: () };
    /// let mut other = ReprC{ a: [true, false], b: &81u32, c: (), d: () };
    ///
    /// let this_ptr: *mut _ = &mut this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprC::OFFSET_A.swap_nonoverlapping(this_ptr, other_ptr);
    ///     ReprC::OFFSET_B.swap_nonoverlapping(this_ptr, other_ptr);
    /// }
    /// assert_eq!( this.a, [true, false] );
    /// assert_eq!( this.b, &81 );
    ///
    /// assert_eq!( other.a, [false, true] );
    /// assert_eq!( other.b, &27 );
    ///
    /// ```
    ///
    #[inline(always)]
    pub unsafe fn swap_nonoverlapping(self, left: *mut S, right: *mut S) {
        core::ptr::swap_nonoverlapping::<F>(
            get_mut_ptr_method!(self, left, F),
            get_mut_ptr_method!(self, right, F),
            1,
        )
    }

    /// Swaps the values of a field between `left` and `right`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprC;
    ///
    /// let mut this = ReprC{ a: [true, true], b: 0x0Fu8, c: (), d: () };
    /// let mut other = ReprC{ a: [false, false], b: 0xF0u8, c: (), d: () };
    ///
    /// ReprC::OFFSET_A.swap_mut(&mut this, &mut other);
    /// ReprC::OFFSET_B.swap_mut(&mut this, &mut other);
    ///
    /// assert_eq!( this.a, [false, false] );
    /// assert_eq!( this.b, 0xF0u8 );
    ///
    /// assert_eq!( other.a, [true, true] );
    /// assert_eq!( other.b, 0x0Fu8 );
    ///
    /// ```
    ///
    #[inline(always)]
    pub fn swap_mut(self, left: &mut S, right: &mut S) {
        unsafe {
            core::mem::swap(
                &mut *get_mut_ptr_method!(self, left, F),
                &mut *get_mut_ptr_method!(self, right, F),
            )
        }
    }
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Copies the unaligned field that this is an offset for.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: Some(false), b: [8i32, 13, 21], c: (), d: () };
    ///
    /// assert_eq!( ReprPacked::OFFSET_A.get_copy(&this), Some(false) );
    /// assert_eq!( ReprPacked::OFFSET_B.get_copy(&this), [8i32, 13, 21] );
    ///
    /// ```
    ///
    /// This method can't be called for non-Copy fields.
    /// ```compile_fail
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: vec![0, 1, 2], b: (), c: (), d: () };
    ///
    /// let _ = ReprPacked::OFFSET_A.get_copy(&this);
    ///
    /// ```
    #[inline(always)]
    pub fn get_copy(self, base: &S) -> F
    where
        F: Copy,
    {
        unsafe { get_ptr_method!(self, base, F).read_unaligned() }
    }

    /// Copies the unaligned field that this is an offset for.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read_unaligned`](https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: 10u8, b: "20", c: (), d: () };
    ///
    /// let ptr: *const _ = &this;
    /// unsafe{
    ///     assert_eq!( ReprPacked::OFFSET_A.read_copy(ptr), 10u8 );
    ///     assert_eq!( ReprPacked::OFFSET_B.read_copy(ptr), "20" );
    /// }
    /// ```
    ///
    /// This method can't be called for non-Copy fields.
    /// ```compile_fail
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// let this = ReprPacked{ a: vec![0, 1, 2], b: "20", c: (), d: () };
    ///
    /// unsafe{
    ///     let _ = ReprPacked::OFFSET_A.read_copy(&this);
    /// }
    /// ```
    #[inline(always)]
    pub unsafe fn read_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        get_ptr_method!(self, base, F).read_unaligned()
    }

    /// Reads the value from the field in `source` without moving it.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read_unaligned`](https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    ///
    /// use std::mem::ManuallyDrop;
    ///
    /// let this = ManuallyDrop::new(ReprPacked{
    ///     a: vec![0, 1, 2],
    ///     b: "20".to_string(),
    ///     c: (),
    ///     d: (),
    /// });
    ///
    /// let ptr: *const _ = &*this;
    /// unsafe{
    ///     assert_eq!( ReprPacked::OFFSET_A.read(ptr), vec![0, 1, 2] );
    ///     assert_eq!( ReprPacked::OFFSET_B.read(ptr), "20".to_string() );
    /// }
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn read(self, source: *const S) -> F {
        get_ptr_method!(self, source, F).read_unaligned()
    }

    /// Writes `value` ìnto the field in `source` without dropping the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::write_unaligned`](https://doc.rust-lang.org/std/ptr/fn.write_unaligned.html).
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: 10u8, b: "20", c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    /// unsafe{
    ///     ReprPacked::OFFSET_A.write(ptr, 13u8);
    ///     ReprPacked::OFFSET_B.write(ptr, "21");
    /// }
    /// assert_eq!( moved(this.a), 13u8 );
    /// assert_eq!( moved(this.b), "21" );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn write(self, source: *mut S, value: F) {
        get_mut_ptr_method!(self, source, F).write_unaligned(value)
    }

    /// Copies the field in `source` into `destination`.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::copy`](https://doc.rust-lang.org/std/ptr/fn.copy.html),
    /// except that `source` and `destination` do not need to be properly aligned.
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    ///
    /// let this = ReprPacked{ a: 10u8, b: "20", c: (), d: () };
    /// let mut other = ReprPacked{ a: 0u8, b: "", c: (), d: () };
    ///
    /// let this_ptr: *const _ = &this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprPacked::OFFSET_A.copy(this_ptr, other_ptr);
    ///     ReprPacked::OFFSET_B.copy(this_ptr, other_ptr);
    /// }
    /// assert_eq!( moved(this.a), 10u8 );
    /// assert_eq!( moved(this.b), "20" );
    ///
    /// assert_eq!( moved(other.a), 10u8 );
    /// assert_eq!( moved(other.b), "20" );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn copy(self, source: *const S, destination: *mut S) {
        core::ptr::copy(
            get_ptr_method!(self, source, F) as *const u8,
            get_mut_ptr_method!(self, destination, F) as *mut u8,
            Mem::<F>::SIZE,
        );
    }

    /// Copies the field in `source` into `destination`,
    /// `source` and `destination` must not overlap.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::copy_nonoverlapping`
    /// ](https://doc.rust-lang.org/std/ptr/fn.copy_nonoverlapping.html),
    /// except that `source` and `destination` do not need to be properly aligned.
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let this = ReprPacked{ a: '#', b: 81, c: (), d: () };
    /// let mut other = ReprPacked{ a: '_', b: 0, c: (), d: () };
    ///
    /// let this_ptr: *const _ = &this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprPacked::OFFSET_A.copy_nonoverlapping(this_ptr, other_ptr);
    ///     ReprPacked::OFFSET_B.copy_nonoverlapping(this_ptr, other_ptr);
    /// }
    /// assert_eq!( moved(this.a), '#' );
    /// assert_eq!( moved(this.b), 81 );
    ///
    /// assert_eq!( moved(other.a), '#' );
    /// assert_eq!( moved(other.b), 81 );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn copy_nonoverlapping(self, source: *const S, destination: *mut S) {
        core::ptr::copy_nonoverlapping(
            get_ptr_method!(self, source, F) as *const u8,
            get_mut_ptr_method!(self, destination, F) as *mut u8,
            Mem::<F>::SIZE,
        );
    }
}

macro_rules! replace_unaligned {
    ($self:ident, $base:expr, $value:expr, $F:ty) => {{
        let ptr = get_mut_ptr_method!($self, $base, $F);
        let ret = ptr.read_unaligned();
        ptr.write_unaligned($value);
        ret
    }};
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Replaces the value of a field in `dest` with `value`,
    /// returning the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::replace`](https://doc.rust-lang.org/std/ptr/fn.replace.html),
    /// except that `dest` does not need to be properly aligned.
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: [0u8, 1], b: false, c: (), d: () };
    ///
    /// let ptr: *mut _ = &mut this;
    /// unsafe{
    ///     assert_eq!( ReprPacked::OFFSET_A.replace(ptr, [2, 3]), [0u8, 1] );
    ///     assert_eq!( ReprPacked::OFFSET_B.replace(ptr, true), false );
    /// }
    ///
    /// assert_eq!( moved(this.a), [2u8, 3] );
    /// assert_eq!( moved(this.b), true );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn replace(self, dest: *mut S, value: F) -> F {
        replace_unaligned!(self, dest, value, F)
    }

    /// Replaces the value of a field in `dest` with `value`,
    /// returning the old value of the field.
    ///
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: [0u8, 1], b: false, c: (), d: () };
    ///
    /// assert_eq!( ReprPacked::OFFSET_A.replace_mut(&mut this, [2, 3]), [0u8, 1] );
    /// assert_eq!( ReprPacked::OFFSET_B.replace_mut(&mut this, true), false );
    ///
    /// assert_eq!( moved(this.a), [2u8, 3] );
    /// assert_eq!( moved(this.b), true );
    ///
    /// ```
    pub fn replace_mut(self, dest: &mut S, value: F) -> F {
        unsafe { replace_unaligned!(self, dest, value, F) }
    }
}

macro_rules! unaligned_swap {
    ($self:ident, $left:ident, $right:ident, $left_to_right:expr, $F:ty) => {{
        // This function can definitely be optimized.
        let mut tmp = MaybeUninit::<$F>::uninit();
        let tmp = tmp.as_mut_ptr() as *mut u8;

        let $left = get_mut_ptr_method!($self, $left, $F) as *mut u8;
        let $right = get_mut_ptr_method!($self, $right, $F) as *mut u8;
        core::ptr::copy_nonoverlapping($left, tmp, Mem::<$F>::SIZE);
        $left_to_right($right, $left, Mem::<$F>::SIZE);
        core::ptr::copy_nonoverlapping(tmp, $right, Mem::<$F>::SIZE);
    }};
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Swaps the values of a field between the `left` and `right` pointers.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::swap`](https://doc.rust-lang.org/std/ptr/fn.swap.html),
    /// except that it does not require aligned pointers.
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: '=', b: 64u16, c: (), d: () };
    /// let mut other = ReprPacked{ a: '!', b: 255u16, c: (), d: () };
    ///
    /// let this_ptr: *mut _ = &mut this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprPacked::OFFSET_A.swap(this_ptr, other_ptr);
    ///     ReprPacked::OFFSET_B.swap(this_ptr, other_ptr);
    /// }
    /// assert_eq!( moved(this.a), '!' );
    /// assert_eq!( moved(this.b), 255 );
    ///
    /// assert_eq!( moved(other.a), '=' );
    /// assert_eq!( moved(other.b), 64 );
    ///
    /// ```
    #[inline(always)]
    pub unsafe fn swap(self, left: *mut S, right: *mut S) {
        unaligned_swap!(self, left, right, core::ptr::copy, F)
    }

    /// Swaps the values of a field between the non-overlapping `left` and `right` pointers.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::swap_nonoverlapping`
    /// ](https://doc.rust-lang.org/std/ptr/fn.swap_nonoverlapping.html)
    /// except that it does not require aligned pointers.
    ///
    /// Those safety requirements only apply to the field that this is an offset for,
    /// fields after it or before it don't need to be valid to call this method.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: [false, true], b: &27u32, c: (), d: () };
    /// let mut other = ReprPacked{ a: [true, false], b: &81u32, c: (), d: () };
    ///
    /// let this_ptr: *mut _ = &mut this;
    /// let other_ptr: *mut _ = &mut other;
    /// unsafe{
    ///     ReprPacked::OFFSET_A.swap_nonoverlapping(this_ptr, other_ptr);
    ///     ReprPacked::OFFSET_B.swap_nonoverlapping(this_ptr, other_ptr);
    /// }
    /// assert_eq!( moved(this.a), [true, false] );
    /// assert_eq!( moved(this.b), &81 );
    ///
    /// assert_eq!( moved(other.a), [false, true] );
    /// assert_eq!( moved(other.b), &27 );
    ///
    /// ```
    ///
    #[inline(always)]
    pub unsafe fn swap_nonoverlapping(self, left: *mut S, right: *mut S) {
        unaligned_swap!(self, left, right, core::ptr::copy_nonoverlapping, F)
    }

    /// Swaps the values of a field between `left` and `right`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #![deny(safe_packed_borrows)]
    /// use repr_offset::for_examples::ReprPacked;
    /// use repr_offset::utils::moved;
    ///
    /// let mut this = ReprPacked{ a: [true, true], b: 0x0Fu8, c: (), d: () };
    /// let mut other = ReprPacked{ a: [false, false], b: 0xF0u8, c: (), d: () };
    ///
    /// ReprPacked::OFFSET_A.swap_mut(&mut this, &mut other);
    /// ReprPacked::OFFSET_B.swap_mut(&mut this, &mut other);
    ///
    /// assert_eq!( moved(this.a), [false, false] );
    /// assert_eq!( moved(this.b), 0xF0u8 );
    ///
    /// assert_eq!( moved(other.a), [true, true] );
    /// assert_eq!( moved(other.b), 0x0Fu8 );
    ///
    /// ```
    ///
    #[inline(always)]
    pub fn swap_mut(self, left: &mut S, right: &mut S) {
        // This function could probably be optimized.
        unsafe {
            let left = get_mut_ptr_method!(self, left, F);
            let right = get_mut_ptr_method!(self, right, F);
            let tmp = left.read_unaligned();
            left.write_unaligned(right.read_unaligned());
            right.write_unaligned(tmp);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types_for_tests::StructPacked;

    use core::mem;

    #[test]
    fn test_constructor_offset() {
        unsafe {
            let field_0 = FieldOffset::<(u128,), u8, Aligned>::new(0);
            let field_1 = field_0.next_field_offset::<u32, Aligned>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), mem::align_of::<u32>());
        }
        unsafe {
            let field_0 = FieldOffset::<StructPacked<u128, (), (), ()>, u8, Unaligned>::new(0);
            let field_1 = field_0.next_field_offset::<u32, Unaligned>();
            let field_2 = field_1.next_field_offset::<&'static str, Unaligned>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), 1);
            assert_eq!(field_2.offset(), 5);
        }
    }
}
