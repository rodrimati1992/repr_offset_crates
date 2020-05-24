/// A const-equivalent of `core::cmp::min::<usize>`
pub const fn min_usize(l: usize, r: usize) -> usize {
    let mask_r = ((l < r) as usize).wrapping_sub(1);
    (r & mask_r) | (l & !mask_r)
}

/// Helper type with associated constants for `core::mem` functions (and a few more).
pub struct Mem<T>(T);

impl<T> Mem<T> {
    /// Equivalent to `core::mem::size_of`.
    pub const SIZE: usize = core::mem::size_of::<T>();

    /// Equivalent to `core::mem::align_of`.
    pub const ALIGN: usize = core::mem::align_of::<T>();
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

#[cfg(any(not(rust_1_36), test))]
mod maybeuninit_emulation;

#[cfg(rust_1_36)]
pub(crate) type UnalignedMaybeUninit<T> = core::mem::MaybeUninit<T>;

#[cfg(not(rust_1_36))]
pub(crate) type UnalignedMaybeUninit<T> = self::maybeuninit_emulation::UnalignedMaybeUninit<T>;
