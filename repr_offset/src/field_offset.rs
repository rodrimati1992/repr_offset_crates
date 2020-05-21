/*!
Utilities for computing the layout of fields in types with a stable layout.
*/

////////////////////////////////////////////////////////////////////////////////

mod struct_field_offset;

pub use self::struct_field_offset::FieldOffset;

////////////////////////////////////////////////////////////////////////////////

/// A marker type representing that a type's fields are aligned.
#[derive(Debug, Copy, Clone)]
pub struct Aligned;

/// A marker type representing that a type has packed fields,
/// which are potentially unaligned.
#[derive(Debug, Copy, Clone)]
pub struct Packed;
