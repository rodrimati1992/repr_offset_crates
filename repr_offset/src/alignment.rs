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
pub type CombinePackingOut<Lhs, Rhs> = <Lhs as CombinePacking<Rhs>>::Output;

/// Trait that combines two [`Alignment`] types,
/// determines the return type of `FieldOffset + FieldOffset`.
///
/// [`Alignment`]: ./trait.Alignment.html
pub trait CombinePacking<Rhs: Alignment> {
    /// This is [`Aligned`] if both `Self` and the `Rhs` parameter are [`Aligned`],
    /// otherwise it is [`Unaligned`].
    ///
    /// [`Alignment`]: ./trait.Alignment.html
    /// [`Aligned`]:  ./struct.Aligned.html
    /// [`Unaligned`]: ./struct.Unaligned.html
    type Output: Alignment;
}

impl<A: Alignment> CombinePacking<A> for Aligned {
    type Output = A;
}
impl<A: Alignment> CombinePacking<A> for Unaligned {
    type Output = Unaligned;
}

type Al = Aligned;
type Un = Unaligned;

macro_rules! tuple_impls {
    (small=>
        $( $ty:ty = $output:ty ,)*
    ) => {
        $(
            impl<Carry: Alignment> CombinePacking<Carry> for $ty {
                type Output = $output;
            }
        )*
    };
    (large=>
        $((
            $( ($t0:ident,$t1:ident,$t2:ident,$t3:ident,), )*
            $($trailing:ident,)*
        ))*
    )=>{
        $(
            #[allow(non_camel_case_types)]
            impl<A: Alignment, $($t0,$t1,$t2,$t3,)* $($trailing,)* CombTuples >
                CombinePacking<A>
            for ($($t0,$t1,$t2,$t3,)* $($trailing,)*)
            where
                ($($trailing,)*): CombinePacking<A>,
                $( ($t0,$t1,$t2,$t3): CombinePacking<Aligned>, )*
                (
                    $( CombinePackingOut<($t0,$t1,$t2,$t3), Aligned>, )*
                ):CombinePacking<
                    CombinePackingOut<($($trailing,)*), A>,
                    Output = CombTuples,
                >,
                CombTuples: Alignment,
            {
                type Output = CombTuples;
            }
        )*
    };
}

/*
fn main() {
    fn as_ty(b:bool)->&'static str{
        if b {"Un"}else{"Al"}
    }

    for elem_count in 0..=4 {
        for bits in 0..1<<elem_count {
            let is_optional=(0..elem_count)
                .map(|i| (bits>>i)&1!=0 )
                .collect::<Vec<bool>>();

            let tup=is_optional.iter().copied().map(as_ty).collect::<Vec<_>>();
            let any_optional=is_optional.iter().cloned().any(|x|x);

            println!(
                "({tup})={output},",
                tup=tup.join(","),
                output=as_ty(any_optional),
            )
        }
    }
}
*/

tuple_impls! {
    small=>
    ()=Carry,
    (Al,)=Carry,
    (Un,)=Un,
    (Al,Al)=Carry,
    (Un,Al)=Un,
    (Al,Un)=Un,
    (Un,Un)=Un,
    (Al,Al,Al)=Carry,
    (Un,Al,Al)=Un,
    (Al,Un,Al)=Un,
    (Un,Un,Al)=Un,
    (Al,Al,Un)=Un,
    (Un,Al,Un)=Un,
    (Al,Un,Un)=Un,
    (Un,Un,Un)=Un,
    (Al,Al,Al,Al)=Carry,
    (Un,Al,Al,Al)=Un,
    (Al,Un,Al,Al)=Un,
    (Un,Un,Al,Al)=Un,
    (Al,Al,Un,Al)=Un,
    (Un,Al,Un,Al)=Un,
    (Al,Un,Un,Al)=Un,
    (Un,Un,Un,Al)=Un,
    (Al,Al,Al,Un)=Un,
    (Un,Al,Al,Un)=Un,
    (Al,Un,Al,Un)=Un,
    (Un,Un,Al,Un)=Un,
    (Al,Al,Un,Un)=Un,
    (Un,Al,Un,Un)=Un,
    (Al,Un,Un,Un)=Un,
    (Un,Un,Un,Un)=Un,
}

/*
fn main() {
    fn as_ty(b:bool)->&'static str{
        if b {"Un"}else{"Al"}
    }
    let tup_size=4;

    for elem_count in 5..=12 {
        print!("(");
        for which_tup in 0..elem_count/tup_size {
            let start=which_tup*tup_size;
            print!("(");
            for e in start..start+tup_size{
                print!("A{},",e);
            }
            print!("),");
        }
        for e in elem_count/tup_size*tup_size..elem_count{
            print!("A{},",e);
        }
        println!(")");
    }
}

*/

tuple_impls! {
    large=>
    ((A0,A1,A2,A3,),A4,)
    ((A0,A1,A2,A3,),A4,A5,)
    ((A0,A1,A2,A3,),A4,A5,A6,)
    ((A0,A1,A2,A3,),(A4,A5,A6,A7,),)
    ((A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,)
    ((A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,A9,)
    ((A0,A1,A2,A3,),(A4,A5,A6,A7,),A8,A9,A10,)
    ((A0,A1,A2,A3,),(A4,A5,A6,A7,),(A8,A9,A10,A11,),)
}
