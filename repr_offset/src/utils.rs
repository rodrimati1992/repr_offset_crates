//! Miscelaneous functions.

use core::marker::PhantomData;

/// A helper function to force a variable to move (copy if it's a Copy type).
///
/// # Example
///
/// ```rust
/// use repr_offset::utils::moved;
///
/// #[repr(C, packed)]
/// struct Packed{
///     foo: usize,
///     bar: u64,
/// }
///
/// let this = Packed{ foo: 21, bar: 34 };
///
/// assert_eq!( moved(this.foo), 21 );
/// assert_eq!( moved(this.bar), 34 );
///
/// // The code below causes undefined behavior because:
/// // -`assert_eq` borrows the operands implicitly.
/// // - Fields of `#[repr(C, packed)]` structs create unaligned references when borrowed.
/// // - Unaligned references are undefined behavior.
/// //
/// // unsafe{
/// //      assert_eq!( this.foo, 21 );
/// //      assert_eq!( this.bar, 34 );
/// // }
///
/// ```
#[inline(always)]
pub const fn moved<T>(val: T) -> T {
    val
}

/// A const-equivalent of `core::cmp::min::<usize>`
pub(crate) const fn min_usize(l: usize, r: usize) -> usize {
    let mask_r = ((l < r) as usize).wrapping_sub(1);
    (r & mask_r) | (l & !mask_r)
}

/// Helper type with associated constants for `core::mem` functions (and a few more).
pub(crate) struct Mem<T>(T);

impl<T> Mem<T> {
    /// Equivalent to `core::mem::size_of`.
    pub const SIZE: usize = core::mem::size_of::<T>();

    /// Equivalent to `core::mem::align_of`.
    pub const ALIGN: usize = core::mem::align_of::<T>();
}

/// Helper type to construct certain PhantomData in const fns.
pub struct MakePhantomData<T>(T);

impl<T> MakePhantomData<T> {
    /// Constructs a `PhantomData<fn()->T>`,
    /// this is a workaround for constructing them inside `const fn`.
    pub const FN_RET: PhantomData<fn() -> T> = PhantomData;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_min_usize() {
        let max = usize::max_value();
        for l in (0usize..10).chain(max - 10..=max) {
            for r in (0usize..10).chain(max - 10..=max) {
                assert_eq!(core::cmp::min(l, r), min_usize(l, r),);
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
#[repr(transparent)]
pub struct AsPhantomDataFn<'s, T> {
    pub reference: &'s T,
    pub ty: PhantomData<fn() -> T>,
}

////////////////////////////////////////////////////////////////////////////////

#[doc(hidden)]
pub trait AsPhantomData: Sized {
    #[doc(hidden)]
    const __REPR_OFFSET_PHANTOMDATA: PhantomData<Self> = PhantomData;

    #[doc(hidden)]
    const __REPR_OFFSET_PHANTOMDATA_FN: PhantomData<fn() -> Self> = PhantomData;
}

impl<T> AsPhantomData for T {}

////////////////////////////////////////////////////////////////////////////////

/// Gets the type pointed-to by a pointer.
pub unsafe trait PointerTarget {
    /// The pointed-to type.
    type Target;
}

unsafe impl<T> PointerTarget for &T {
    type Target = T;
}

unsafe impl<T> PointerTarget for &mut T {
    type Target = T;
}

unsafe impl<T> PointerTarget for *const T {
    type Target = T;
}

unsafe impl<T> PointerTarget for *mut T {
    type Target = T;
}
