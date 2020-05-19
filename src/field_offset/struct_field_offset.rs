use crate::{Aligned, Unaligned};

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
/// - `A`: Whether the field is aligned (represented with the `Aligned` marker type)
/// or not (represented with the `Unaligned` marker type).
/// This changes which methods are available,and the implementation of some of them.
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

impl<S, F, A> FieldOffset<S, F, A> {
    /// Constructs this `FieldOffset` from the offset of the field.
    ///
    /// # Safety
    ///
    /// Callers must ensure all of these:
    ///
    /// - `offset` must be the byte offset of a field of type `F` inside the struct `S`.
    ///
    /// - The `A` type parameter must be [`Unaligned`]
    /// if the field of type `F` could be unaligned,
    /// and [`Aligned`] if it is definitely aligned.
    ///
    /// [`Aligned`]: ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
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
        let offset = super::next_field_offset::<F, Next>(self.offset(), mem::align_of::<S>());
        FieldOffset::new(offset)
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
        unsafe { (base as *const u8).offset(self.offset as isize) as *const F }
    }
    /// Copies the field that this is an offset for,from a potentially unaligned field.
    #[inline(always)]
    pub fn get_copy_unaligned(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { self.get_raw(base).read_unaligned() }
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Gets a reference to the field that this is an offset for.
    #[inline(always)]
    pub fn get(self, base: &S) -> &F {
        unsafe { &*self.get_raw(base) }
    }

    /// Copies the aligned field that this is an offset for.
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { *self.get_raw(base) }
    }
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Copies the unaligned field that this is an offset for.
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { self.get_raw(base).read_unaligned() }
    }
}
