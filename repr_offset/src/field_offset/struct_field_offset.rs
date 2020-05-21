use crate::{offset_calc::GetNextFieldOffset, Aligned, Packed};

use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    mem,
};

/// Represents the offset of a field inside a type.
///
/// # Type parameters
///
/// The type parameters are:
///
/// - `S`: the struct that contains the field that this is an offset for.
///
/// - `F`: the type of the field this is an offset for.
///
/// - The `A` type parameter is [`Packed`]
/// if the `S` struct is `#[repr(C,packed)]`, or [`Aligned`] if it's not packed.
/// This changes which methods are available,and the implementation of some of them.
///
/// # Examples
///
/// ### No Macros
///
/// This example demonstrates how you can construct `FieldOffset` without macros.
///
/// You can use the [`unsafe_struct_field_offsets`] macro to construct the constants
/// more conveniently.
///
/// ```rust
/// use repr_offset::{Aligned, FieldOffset};
///
/// use std::mem;
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
///     const OFFSET_THIRD: FieldOffset<Self, Option<T>, Aligned> = unsafe{
///         Self::OFFSET_SECOND.next_field_offset()
///     };
/// }
///
/// ```
///
/// [`unsafe_struct_field_offsets`]: ./macro.unsafe_struct_field_offsets.html
///
#[repr(transparent)]
pub struct FieldOffset<S, F, A> {
    offset: usize,
    _marker: PhantomData<DummyType<(S, F, A)>>,
}

// Workaround for `PhantomData<fn()->T>` not being constructible in const contexts
struct DummyType<T>(fn() -> T);

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
// caused by delegating to `get_raw`
macro_rules! get_raw_method {
    ($self:ident, $base:expr, $F:ty) => {
        (($base as *const _ as *const u8).offset($self.offset as isize) as *const $F)
    };
}

// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `get_raw_mut`
macro_rules! get_raw_mut_method {
    ($self:ident, $base:expr, $F:ty) => {
        (($base as *mut _ as *mut u8).offset($self.offset as isize) as *mut $F)
    };
}

impl<S, F, A> FieldOffset<S, F, A> {
    /// Constructs this `FieldOffset` from the offset of the field.
    ///
    /// # Safety
    ///
    /// Callers must ensure all of these:
    ///
    /// - `S` must be a `#[repr(C)]` struct.
    ///
    /// - `offset` must be the byte offset of a field of type `F` inside the struct `S`.
    ///
    /// - The `A` type parameter must be [`Packed`]
    /// if the `S` struct is `#[repr(C,packed)]`, or [`Aligned`] if it's not packed.
    ///
    /// [`Aligned`]: ./struct.Aligned.html
    /// [`Packed`]: ./struct.Packed.html
    ///
    #[inline(always)]
    pub const unsafe fn new(offset: usize) -> Self {
        Self {
            offset,
            _marker: PhantomData,
        }
    }

    /// Constructs a `FieldOffset` by calculating the offset of the next field.
    ///
    /// # Safety
    ///
    /// Callers must ensure that `Next` is the type of the field after the one that
    /// this is an offset for.
    pub const unsafe fn next_field_offset<Next>(self) -> FieldOffset<S, Next, A> {
        let offset = GetNextFieldOffset {
            previous_offset: self.offset,
            container_alignment: mem::align_of::<S>(),
            size_of_previous: mem::size_of::<F>(),
            align_of_next: mem::align_of::<Next>(),
        }
        .call();

        FieldOffset {
            offset,
            _marker: PhantomData,
        }
    }
}

impl<S, F, A> FieldOffset<S, F, A> {
    /// The offset of the `F` field in the `S` struct.
    #[inline(always)]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Gets a raw pointer to the field that this is an offset for.
    #[inline(always)]
    pub fn get_raw(self, base: *const S) -> *const F {
        unsafe { get_raw_method!(self, base, F) }
    }

    /// Gets a mutable raw pointer to the field that this is an offset for.
    #[inline(always)]
    pub fn get_raw_mut(self, base: *mut S) -> *mut F {
        unsafe { get_raw_mut_method!(self, base, F) }
    }

    /// Copies the field that this is an offset for,from a potentially unaligned field.
    #[inline(always)]
    pub fn get_copy_unaligned(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { get_raw_method!(self, base, F).read_unaligned() }
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Gets a reference to the field that this is an offset for.
    #[inline(always)]
    pub fn get(self, base: &S) -> &F {
        unsafe { &*get_raw_method!(self, base, F) }
    }

    /// Gets a mutable reference to the field that this is an offset for.
    #[inline(always)]
    pub fn get_mut(self, base: &mut S) -> &mut F {
        unsafe { &mut *get_raw_mut_method!(self, base, F) }
    }

    /// Copies the aligned field that this is an offset for.
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { *get_raw_method!(self, base, F) }
    }
}

impl<S, F> FieldOffset<S, F, Packed> {
    /// Copies the unaligned field that this is an offset for.
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { get_raw_method!(self, base, F).read_unaligned() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::types_for_tests::StructPacked;

    #[test]
    fn test_constructor_offset() {
        unsafe {
            let field_0 = FieldOffset::<(u128,), u8, Aligned>::new(0);
            let field_1 = field_0.next_field_offset::<u32>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), mem::align_of::<u32>());
        }
        unsafe {
            let field_0 = FieldOffset::<StructPacked<u128, (), (), ()>, u8, Packed>::new(0);
            let field_1 = field_0.next_field_offset::<u32>();
            let field_2 = field_1.next_field_offset::<&'static str>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), 1);
            assert_eq!(field_2.offset(), 5);
        }
    }
}
