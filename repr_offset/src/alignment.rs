//! Type-level encoding of `enum Alignment { Aligned, Unaligned }`

/// A marker type representing that a `FieldOffset` is for an aligned field.
#[derive(Debug, Copy, Clone)]
pub struct Aligned;

/// A marker type representing that a `FieldOffset` is for a (potentially) unaligned field.
#[derive(Debug, Copy, Clone)]
pub struct Unaligned;

mod sealed {
    use super::{Aligned, Unaligned};
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

/// Combines two [`Alignment`] types,
/// determines the return type of `FieldOffset + FieldOffset`.
///
/// [`Alignment`]: ./trait.Alignment.html
/// [`FieldOffset + FieldOffset`]: ./struct.FieldOffset.html#impl-Add<FieldOffset<F%2C F2%2C A2>>
pub type CombineAlignmentOut<Lhs, Rhs> = <Lhs as CombineAlignment<Rhs>>::Output;

/// Trait that combines two [`Alignment`] types,
/// determines the return type of `FieldOffset + FieldOffset`.
///
/// [`Alignment`]: ./trait.Alignment.html
pub trait CombineAlignment<Rhs: Alignment> {
    /// This is [`Aligned`] if both `Self` and the `Rhs` parameter are [`Aligned`],
    /// otherwise it is [`Unaligned`].
    ///
    /// [`Alignment`]: ./trait.Alignment.html
    /// [`Aligned`]:  ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
    type Output: Alignment;
}

impl<A: Alignment> CombineAlignment<A> for Aligned {
    type Output = A;
}
impl<A: Alignment> CombineAlignment<A> for Unaligned {
    type Output = Unaligned;
}

macro_rules! tuple_impls {
    (small=> $ty:ty = $output:ty ) => {
        impl<Carry: Alignment> CombineAlignment<Carry> for $ty {
            type Output = $output;
        }
    };
    (large=>
        $( ($t0:ident,$t1:ident,$t2:ident,$t3:ident,), )*
        $($trailing:ident,)*
    )=>{
        #[allow(non_camel_case_types)]
        impl<A: Alignment, $($t0,$t1,$t2,$t3,)* $($trailing,)* CombTuples >
            CombineAlignment<A>
        for ($($t0,$t1,$t2,$t3,)* $($trailing,)*)
        where
            ($($trailing,)*): CombineAlignment<A>,
            $( ($t0,$t1,$t2,$t3): CombineAlignment<Aligned>, )*
            (
                $( CombineAlignmentOut<($t0,$t1,$t2,$t3), Aligned>, )*
            ):CombineAlignment<
                CombineAlignmentOut<($($trailing,)*), A>,
                Output = CombTuples,
            >,
            CombTuples: Alignment,
        {
            type Output = CombTuples;
        }
    };
}

impl_all_trait_for_tuples! {
    macro = tuple_impls,
    true = Aligned,
    false = Unaligned,
}
