//! Functions for calculating field offsets.

use crate::utils;

use core::mem;

/// Calculates the offset of a field,given the previous field.
///
/// # Parameters
///
/// `Struct` is the struct that contains the field whose offset this calculates.
///
/// `Prev` is the type of the previous field.
///
/// `Next` is the type of the field whose offset this calculates.
///
/// `previous_offset` is the offset in bytes of the previous field,of `Prev` type.
///
#[inline(always)]
pub const fn next_field_offset<Struct, Prev, Next>(previous_offset: usize) -> usize {
    GetNextFieldOffset {
        previous_offset,
        container_alignment: mem::align_of::<Struct>(),
        size_of_previous: mem::size_of::<Prev>(),
        align_of_next: mem::align_of::<Next>(),
    }
    .call()
}

/// Calculates the offset (in bytes) of a field, with the `call` method.
pub struct GetNextFieldOffset {
    /// The offset in bytes of the previous field.
    pub previous_offset: usize,
    /// The alignment of the type that contains the field.
    pub container_alignment: usize,
    /// the size of the previous field.
    pub size_of_previous: usize,
    /// The alignment of the field.
    pub align_of_next: usize,
}

impl GetNextFieldOffset {
    /// Calculates the offset (in bytes) of a field.
    pub const fn call(&self) -> usize {
        let middle_offset = self.previous_offset + self.size_of_previous;
        let padding = {
            let alignment = utils::min_usize(self.align_of_next, self.container_alignment);
            let misalignment = middle_offset % alignment;

            // Workaround for `if` in const contexts not being stable on Rust 1.34
            let mask = ((misalignment == 0) as usize).wrapping_sub(1);
            (alignment - misalignment) & mask
        };
        middle_offset + padding
    }
}
