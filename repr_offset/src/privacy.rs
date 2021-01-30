//! Type-level encoding of `enum Privacy { IsPublic, IsPrivate }`

/// A marker type representing that a `FieldOffsetWithVis` is for a public field.
#[derive(Debug, Copy, Clone)]
pub struct IsPublic;

/// A marker type representing that a `FieldOffsetWithVis` is for a private field.
#[derive(Debug, Copy, Clone)]
pub struct IsPrivate;

mod sealed {
    use super::{IsPrivate, IsPublic};
    pub trait Sealed {}

    impl Sealed for IsPublic {}
    impl Sealed for IsPrivate {}
}
use self::sealed::Sealed;

/// Marker trait for types that represents the privacy of a `FieldOffsetWithVis`.
///
/// This is only implemented by [`IsPublic`] and [`IsPrivate`]
///
/// [`IsPublic`]:  ./struct.IsPublic.html
/// [`IsPrivate`]: ./struct.IsPrivate.html
pub trait Privacy: Sealed {}

impl Privacy for IsPublic {}
impl Privacy for IsPrivate {}

/// Combines two [`Privacy`] types.
///
/// This is used to compute the `Privacy` associated type of the `GetFieldOffset` trait in
/// impls for accessing nested fields.
///
/// [`Privacy`]: ./trait.Privacy.html
pub type CombinePrivacyOut<Lhs, Rhs> = <Lhs as CombinePrivacy<Rhs>>::Output;

/// Trait that combines two [`Privacy`] types.
///
/// [`Privacy`]: ./trait.Privacy.html
pub trait CombinePrivacy<Rhs: Privacy> {
    /// This is [`IsPublic`] if both `Self` and the `Rhs` parameter are [`IsPublic`],
    /// otherwise it is [`IsPrivate`].
    ///
    /// [`Privacy`]: ./trait.Privacy.html
    /// [`IsPublic`]:  ./struct.IsPublic.html
    /// [`IsPrivate`]: ./struct.IsPrivate.html
    type Output: Privacy;
}

impl<A: Privacy> CombinePrivacy<A> for IsPublic {
    type Output = A;
}
impl<A: Privacy> CombinePrivacy<A> for IsPrivate {
    type Output = IsPrivate;
}

macro_rules! tuple_impls {
    (small=> $ty:ty = $output:ty) => {
        impl<Carry: Privacy> CombinePrivacy<Carry> for $ty {
            type Output = $output;
        }
    };
    (large=>
        $( ($t0:ident,$t1:ident,$t2:ident,$t3:ident,), )*
        $($trailing:ident,)*
    )=>{
        #[allow(non_camel_case_types)]
        impl<A: Privacy, $($t0,$t1,$t2,$t3,)* $($trailing,)* CombTuples >
            CombinePrivacy<A>
        for ($($t0,$t1,$t2,$t3,)* $($trailing,)*)
        where
            ($($trailing,)*): CombinePrivacy<A>,
            $( ($t0,$t1,$t2,$t3): CombinePrivacy<IsPublic>, )*
            (
                $( CombinePrivacyOut<($t0,$t1,$t2,$t3), IsPublic>, )*
            ):CombinePrivacy<
                CombinePrivacyOut<($($trailing,)*), A>,
                Output = CombTuples,
            >,
            CombTuples: Privacy,
        {
            type Output = CombTuples;
        }
    };
}

impl_all_trait_for_tuples! {
    macro = tuple_impls,
    true = IsPublic,
    false = IsPrivate,
}
