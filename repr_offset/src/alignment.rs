/// A marker type representing that a `FieldOffset` is for an aligned field.
#[derive(Debug, Copy, Clone)]
pub struct Aligned;

/// A marker type representing that a `FieldOffset` is for a (potentially) unaligned field.
#[derive(Debug, Copy, Clone)]
pub struct Unaligned;

mod sealed {
    use super::*;
    pub trait Sealed {}

    impl Sealed for Aligned {}
    impl Sealed for Unaligned {}
}
use self::sealed::Sealed;

/// Marker trait for types that represents the alignment of a `FieldOffset`.
///
/// This is only implemented by [`Aligned`] and [`Unaligned`]
///
/// [`Aligned`]:  ./struct.Aligned.html
/// [`Unaligned`]: ./struct.Unaligned.html
pub trait Alignment: Sealed {}

impl Alignment for Aligned {}
impl Alignment for Unaligned {}

/// Combines two [`Alignment`] types.
///
/// [`Alignment`]: ./trait.Alignment.html
pub type CombinePackingOut<Lhs, Rhs> = <Lhs as CombinePacking<Rhs>>::Output;

/// Trait that combines two [`Alignment`] types.
pub trait CombinePacking<Rhs: Alignment>: Alignment {
    /// This is [`Aligned`] if both `Self` and the `Rhs` parameter are [`Aligned`],
    /// otherwise it is [`Unaligned`].
    ///
    /// [`Alignment`]: ./trait.Alignment.html
    type Output: Alignment;
}

impl<A: Alignment> CombinePacking<A> for Aligned {
    type Output = A;
}
impl<A: Alignment> CombinePacking<A> for Unaligned {
    type Output = Unaligned;
}
