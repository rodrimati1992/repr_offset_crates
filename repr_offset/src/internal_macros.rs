macro_rules! impl_cmp_traits_for_offset {
    ( impl[$($impl_params:tt)*] $self:ty ) => (
        mod __implementing_cmp_traits{
            use super::*;
            use core::cmp::{PartialEq,Eq,PartialOrd,Ord,Ordering};

            impl<$($impl_params)*> PartialEq for $self {
                fn eq(&self, other: &Self)->bool{
                    self.offset==other.offset
                }
            }
            impl<$($impl_params)*> Eq for $self {}

            impl<$($impl_params)*> PartialOrd for $self {
                fn partial_cmp(&self, other: &Self)->Option<Ordering>{
                    self.offset.partial_cmp(&other.offset)
                }
            }

            impl<$($impl_params)*> Ord for $self {
                fn cmp(&self, other: &Self)->Ordering{
                    self.offset.cmp(&other.offset)
                }
            }
        }
    )
}
