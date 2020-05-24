use crate::{
    offset_calc::GetNextFieldOffset,
    utils::{Mem, UnalignedMaybeUninit},
    Aligned, Alignment, CombinePacking, CombinePackingOut, Unaligned,
};

use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Add,
};

/// Represents the offset of a field inside a type.
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
/// Is [`Aligned`] if this offset is aligned for the `F` type, [`Unaligned`] if it's not.
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
// caused by delegating to `get_ptr`
macro_rules! get_ptr_method {
    ($self:ident, $base:expr, $F:ty) => {
        ($base as *const _ as *const u8).offset($self.offset as isize) as *const $F
    };
}

// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `get_mut_ptr`
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
    /// - `S` must be a `#[repr(C)]` struct.
    ///
    /// - `offset` must be the byte offset of a field of type `F` inside the struct `S`.
    ///
    /// - The `A` type parameter must be [`Unaligned`]
    /// if the `S` struct is `#[repr(C,packed)]`, or [`Aligned`] if it's not packed.
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
    /// Callers must ensure that `Next` is the type of the field after the one that
    /// this is an offset for.
    pub const unsafe fn next_field_offset<Next>(self) -> FieldOffset<S, Next, A> {
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
    #[inline(always)]
    pub const fn add<F2, A2>(self, other: FieldOffset<F, F2, A2>) -> FieldOffset<S, F2, A2> {
        FieldOffset::priv_new(self.offset + other.offset)
    }
}

impl<S, F> FieldOffset<S, F, Unaligned> {
    /// Combines this `FieldOffset` with another one, to access a nested field.
    #[inline(always)]
    pub const fn add<F2, A2>(self, other: FieldOffset<F, F2, A2>) -> FieldOffset<S, F2, Unaligned> {
        FieldOffset::priv_new(self.offset + other.offset)
    }
}

/// Equivalent to the inherent `FieldOffset::add` method,
/// which can be ran at compile-time.
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
    /// The offset of the `F` field in the `S` struct.
    #[inline(always)]
    pub const fn offset(self) -> usize {
        self.offset
    }

    /// Changes the `S` type parameter, most useful for `#[repr(transparent)]` wrappers.
    ///
    /// # Safety
    ///
    /// Callers must ensure that there is a field of type `F` at the same offset
    /// inside the `S2` struct
    ///
    pub const unsafe fn cast_struct<S2>(self) -> FieldOffset<S2, F, A> {
        FieldOffset::new(self.offset)
    }

    /// Changes the `F` type parameter.
    ///
    /// # Safety
    ///
    /// Callers must ensure that the `F2` type is compatible with the `F` type.
    pub const unsafe fn cast_field<F2>(self) -> FieldOffset<S, F2, A> {
        FieldOffset::new(self.offset)
    }

    /// Changes this `FieldOffset` to be for a (potentially) unaligned field.
    ///
    /// This is useful if you want to get a field from an unaligned pointer to a
    /// `#[repr(C)]`/`#[repr(C,align())]` struct.
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
    /// Callers must ensure that the offset is a multiple of the alignment of the `F` type.
    pub const unsafe fn to_aligned(self) -> FieldOffset<S, F, Aligned> {
        FieldOffset::new(self.offset)
    }

    /// Gets a raw pointer to a field from a pointer to the `S` struct.
    #[inline(always)]
    pub fn get_ptr(self, base: *const S) -> *const F {
        unsafe { get_ptr_method!(self, base, F) }
    }

    /// Gets a mutable raw pointer to a field from a pointer to the `S` struct.
    #[inline(always)]
    pub fn get_mut_ptr(self, base: *mut S) -> *mut F {
        unsafe { get_mut_ptr_method!(self, base, F) }
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Gets a reference to the field that this is an offset for.
    #[inline(always)]
    pub fn get(self, base: &S) -> &F {
        unsafe { &*get_ptr_method!(self, base, F) }
    }

    /// Gets a mutable reference to the field that this is an offset for.
    #[inline(always)]
    pub fn get_mut(self, base: &mut S) -> &mut F {
        unsafe { &mut *get_mut_ptr_method!(self, base, F) }
    }
}

impl<S, F> FieldOffset<S, F, Aligned> {
    /// Copies the aligned field that this is an offset for.
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { *get_ptr_method!(self, base, F) }
    }

    /// Reads the value from the field in `source` without moving it.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read`](https://doc.rust-lang.org/std/ptr/fn.read.html).
    ///
    #[inline(always)]
    pub unsafe fn read(self, source: *const S) -> F {
        get_ptr_method!(self, source, F).read()
    }

    /// Writes `value` ìnto the field in `source` without dropping the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).
    ///
    #[inline(always)]
    pub unsafe fn write(self, source: *mut S, value: F) {
        get_mut_ptr_method!(self, source, F).write(value)
    }

    /// Copies the field in `source` into `destination`.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::copy`](https://doc.rust-lang.org/std/ptr/fn.copy.html).
    ///
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
    #[inline(always)]
    pub unsafe fn copy_nonoverlapping(self, source: *const S, destination: *mut S) {
        core::ptr::copy_nonoverlapping(
            get_ptr_method!(self, source, F),
            get_mut_ptr_method!(self, destination, F),
            1,
        );
    }

    /// Replaces the value of a field in `dest` with `value`,
    /// returning the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::replace`](https://doc.rust-lang.org/std/ptr/fn.replace.html).
    ///
    #[inline(always)]
    pub unsafe fn replace(self, dest: *mut S, value: F) -> F {
        core::mem::replace(&mut *get_mut_ptr_method!(self, dest, F), value)
    }

    /// Replaces the value of a field in `dest` with `value`,
    /// returning the old value of the field.
    ///
    #[inline(always)]
    pub fn replace_mut(self, dest: &mut S, value: F) -> F {
        unsafe { core::mem::replace(&mut *get_mut_ptr_method!(self, dest, F), value) }
    }

    /// Swaps the values of a field between the `left` and `right` pointers.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::swap`](https://doc.rust-lang.org/std/ptr/fn.swap.html).
    ///
    #[inline(always)]
    pub unsafe fn swap(self, left: *mut S, right: *mut S) {
        core::ptr::swap(
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
    #[inline(always)]
    pub unsafe fn swap_nonoverlapping(self, left: *mut S, right: *mut S) {
        core::ptr::swap(
            get_mut_ptr_method!(self, left, F),
            get_mut_ptr_method!(self, right, F),
        )
    }

    /// Swaps the values of a field between `left` and `right`.
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
    #[inline(always)]
    pub fn get_copy(self, base: *const S) -> F
    where
        F: Copy,
    {
        unsafe { get_ptr_method!(self, base, F).read_unaligned() }
    }

    /// Reads the value from the field in `source` without moving it.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::read`](https://doc.rust-lang.org/std/ptr/fn.read.html),
    /// except that `dest` does not need to be properly aligned.
    ///
    #[inline(always)]
    pub unsafe fn read(self, source: *const S) -> F {
        get_ptr_method!(self, source, F).read_unaligned()
    }

    /// Writes `value` ìnto the field in `source` without dropping the old value of the field.
    ///
    /// # Safety
    ///
    /// This function has the same safety requirements as
    /// [`std::ptr::write`](https://doc.rust-lang.org/std/ptr/fn.write.html).
    ///
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
    /// except that `destination` does not need to be properly aligned.
    ///
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
    /// except that `destination` does not need to be properly aligned.
    ///
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
    #[inline(always)]
    pub unsafe fn replace(self, dest: *mut S, value: F) -> F {
        replace_unaligned!(self, dest, value, F)
    }

    /// Replaces the value of a field in `dest` with `value`,
    /// returning the old value of the field.
    ///
    pub fn replace_mut(self, dest: &mut S, value: F) -> F {
        unsafe { replace_unaligned!(self, dest, value, F) }
    }
}

macro_rules! unaligned_swap {
    ($self:ident, $left:ident, $right:ident, $left_to_right:expr, $F:ty) => {{
        // This function can definitely be optimized.
        let mut tmp = UnalignedMaybeUninit::<$F>::uninit();
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
    /// except that it does not require an aligned pointer.
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
    /// except that it does not require an aligned pointer.
    ///
    #[inline(always)]
    pub unsafe fn swap_nonoverlapping(self, left: *mut S, right: *mut S) {
        unaligned_swap!(self, left, right, core::ptr::copy_nonoverlapping, F)
    }

    /// Swaps the values of a field between `left` and `right`.
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
            let field_1 = field_0.next_field_offset::<u32>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), mem::align_of::<u32>());
        }
        unsafe {
            let field_0 = FieldOffset::<StructPacked<u128, (), (), ()>, u8, Unaligned>::new(0);
            let field_1 = field_0.next_field_offset::<u32>();
            let field_2 = field_1.next_field_offset::<&'static str>();
            assert_eq!(field_0.offset(), 0);
            assert_eq!(field_1.offset(), 1);
            assert_eq!(field_2.offset(), 5);
        }
    }
}
