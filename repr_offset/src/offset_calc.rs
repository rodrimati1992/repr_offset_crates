//! Functions for calculating field offsets.

use crate::utils::{self, Mem};

/// Calculates the offset of a field,given the previous field.
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
