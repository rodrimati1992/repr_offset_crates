//! Functions for calculating field offsets.

use crate::utils::{self, Mem};

/// Calculates the offset of a field in bytes,given the previous field.
///
/// # Parameters
///
/// `Struct` is the struct that contains the field that this calculates the offset for.
///
/// `Prev` is the type of the previous field.
///
/// `Next` is the type of the field that this calculates the offset for.
///
/// `previous_offset` is the offset in bytes of the previous field,of `Prev` type.
///
/// # Example
///
/// ```
/// use repr_offset::offset_calc::next_field_offset;
///
/// #[repr(C, packed)]
/// struct Foo(u8, u16, u32, u64);
///
/// assert_eq!( OFFSET_1, 1 );
/// assert_eq!( OFFSET_2, 3 );
/// assert_eq!( OFFSET_3, 7 );
///
/// const OFFSET_0: usize = 0;
/// const OFFSET_1: usize = next_field_offset::<Foo, u8, u16>(OFFSET_0);
/// const OFFSET_2: usize = next_field_offset::<Foo, u16, u32>(OFFSET_1);
/// const OFFSET_3: usize = next_field_offset::<Foo, u32, u64>(OFFSET_2);
///
/// ```
#[inline(always)]
pub const fn next_field_offset<Struct, Prev, Next>(previous_offset: usize) -> usize {
    GetNextFieldOffset {
        previous_offset,
        previous_size: Mem::<Prev>::SIZE,
        container_alignment: Mem::<Struct>::ALIGN,
        next_alignment: Mem::<Next>::ALIGN,
    }
    .call()
}

/// Calculates the offset (in bytes) of a field, with the `call` method.
///
/// # Example
///
/// ```
/// use repr_offset::offset_calc::GetNextFieldOffset;
///
/// use std::mem;
///
/// #[repr(C, packed)]
/// struct Foo(u8, u16, u32, u64);
///
/// assert_eq!( OFFSET_1, 1 );
/// assert_eq!( OFFSET_2, 3 );
/// assert_eq!( OFFSET_3, 7 );
///
/// const OFFSET_0: usize = 0;
///
/// const OFFSET_1: usize = GetNextFieldOffset{
///     previous_offset: OFFSET_0,
///     previous_size: mem::size_of::<u8>(),
///     container_alignment: mem::align_of::<Foo>(),
///     next_alignment: mem::align_of::<u16>(),
/// }.call();
///
/// const OFFSET_2: usize = GetNextFieldOffset{
///     previous_offset: OFFSET_1,
///     previous_size: mem::size_of::<u16>(),
///     container_alignment: mem::align_of::<Foo>(),
///     next_alignment: mem::align_of::<u32>(),
/// }.call();
///
/// const OFFSET_3: usize = GetNextFieldOffset{
///     previous_offset: OFFSET_2,
///     previous_size: mem::size_of::<u32>(),
///     container_alignment: mem::align_of::<Foo>(),
///     next_alignment: mem::align_of::<u64>(),
/// }.call();
///
/// ```
pub struct GetNextFieldOffset {
    /// The offset in bytes of the previous field.
    pub previous_offset: usize,
    /// The size of the previous field.
    pub previous_size: usize,
    /// The alignment of the type that contains the field.
    pub container_alignment: usize,
    /// The alignment of the field that this calculates the offset for.
    pub next_alignment: usize,
}

impl GetNextFieldOffset {
    /// Calculates the offset (in bytes) of a field.
    pub const fn call(self) -> usize {
        let middle_offset = self.previous_offset + self.previous_size;
        let padding = {
            let alignment = utils::min_usize(self.next_alignment, self.container_alignment);
            let misalignment = middle_offset % alignment;

            // Workaround for `if` in const contexts not being stable on Rust 1.34
            let mask = ((misalignment == 0) as usize).wrapping_sub(1);
            (alignment - misalignment) & mask
        };
        middle_offset + padding
    }
}
