/*!
Utilities for computing the layout of fields in types with a stable layout.
*/

use crate::utils;

use core::mem;

////////////////////////////////////////////////////////////////////////////////

mod struct_field_offset;

pub use self::struct_field_offset::FieldOffset;

////////////////////////////////////////////////////////////////////////////////

/// A marker type representing that a type's fields are aligned.
#[derive(Debug, Copy, Clone)]
pub struct Aligned;

/// A marker type representing that a type's fields are potentially unaligned.
#[derive(Debug, Copy, Clone)]
pub struct Unaligned;

/// Calculates the offset of a field,given the previous field.
///
/// # Parameters
///
/// `Prev` is the type of the previous field.
///
/// `Next` is the type of the field whose offset we are calculating.
///
/// `previous_offset` is the offset in bytes of the previous field,of `Prev` type.
///
/// `container_alignment` is the alignment of the type that contains this field.
#[inline(always)]
pub const fn next_field_offset<Prev, Next>(
    previous_offset: usize,
    container_alignment: usize,
) -> usize {
    GetNextFieldOffset {
        previous_offset,
        container_alignment,
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
    pub const fn call(self) -> usize {
        let Self {
            previous_offset,
            container_alignment,
            size_of_previous,
            align_of_next,
        } = self;
        let middle_offset = previous_offset + size_of_previous;
        let padding = {
            let alignment = utils::min_usize(align_of_next, container_alignment);
            let misalignment = middle_offset % alignment;
            let mask = ((misalignment == 0) as usize).wrapping_sub(1);
            (alignment - misalignment) & mask
        };
        middle_offset + padding
    }
}
