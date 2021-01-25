use crate::{
    alignment::{Aligned, Unaligned},
    ext::{ROExtAcc, ROExtOps, ROExtRawAcc, ROExtRawMutAcc, ROExtRawMutOps, ROExtRawOps},
    FieldOffset,
};

//////////////////////////////////////////////////////////////////////////////

unsafe impl<S> ROExtAcc for S {
    #[inline(always)]
    fn f_get<F>(&self, offset: FieldOffset<Self, F, Aligned>) -> &F {
        unsafe { impl_fo!(fn get<S, F, Aligned>(offset, self)) }
    }
    #[inline(always)]
    fn f_get_mut<F>(&mut self, offset: FieldOffset<Self, F, Aligned>) -> &mut F {
        unsafe { impl_fo!(fn get_mut<S, F, Aligned>(offset, self)) }
    }

    #[inline(always)]
    fn f_get_ptr<F, A>(&self, offset: FieldOffset<Self, F, A>) -> *const F {
        unsafe { impl_fo!(fn get_ptr<S, F, A>(offset, self)) }
    }

    #[inline(always)]
    fn f_get_mut_ptr<F, A>(&mut self, offset: FieldOffset<Self, F, A>) -> *mut F {
        unsafe { impl_fo!(fn get_mut_ptr<S, F, A>(offset, self)) }
    }
}

macro_rules! impl_ROExtOps {
    ($A:ident) => {

        unsafe impl<S> ROExtOps<$A> for S {
            #[inline(always)]
            fn f_replace<F>(&mut self, offset: FieldOffset<Self, F, $A>, value: F) -> F{
                unsafe{ impl_fo!(fn replace_mut<S, F, $A>(offset, self, value)) }
            }

            #[inline(always)]
            fn f_swap<F>(&mut self, offset: FieldOffset<Self, F, $A>, right: &mut S){
                unsafe{ impl_fo!(fn swap_mut<S, F, $A>(offset, self, right)) }

            }

            #[inline(always)]
            fn f_get_copy<F>(&self, offset: FieldOffset<Self, F, $A>) -> F
            where
                F: Copy
            {
                unsafe{ impl_fo!(fn get_copy<S, F, $A>(offset, self)) }
            }
        }
    };
}

impl_ROExtOps! {Aligned}
impl_ROExtOps! {Unaligned}

//////////////////////////////////////////////////////////////////////////////

macro_rules! impl_ROExtRaw {
    ($($ptr:tt)*)=>{
        impl_ROExtRawOps! {Aligned, [$($ptr)*]}
        impl_ROExtRawOps! {Unaligned, [$($ptr)*]}

        unsafe impl<S> ROExtRawAcc for $($ptr)* S {
            #[inline(always)]
            unsafe fn f_raw_get<F, A>(self, offset: FieldOffset<Self::Target, F, A>) -> *const F {
                impl_fo!(fn raw_get<Self::Target, F, A>(offset, self))
            }
        }
    }
}

macro_rules! impl_ROExtRawMut {
    ($($ptr:tt)*)=>{
        impl_ROExtRawMutOps! {Aligned, [$($ptr)*]}
        impl_ROExtRawMutOps! {Unaligned, [$($ptr)*]}

        unsafe impl<S> ROExtRawMutAcc for $($ptr)* S {
            #[inline(always)]
            unsafe fn f_raw_get_mut<F, A>(self, offset: FieldOffset<Self::Target, F, A>) -> *mut F {
                impl_fo!(fn raw_get_mut<Self::Target, F, A>(offset, self))
            }
        }
    }
}

macro_rules! impl_ROExtRawOps {
    ($A:ident, [$($ptr:tt)*])=>{
        unsafe impl<S> ROExtRawOps<$A> for $($ptr)* S {
            #[inline(always)]
            unsafe fn f_read_copy<F>(self, offset: FieldOffset<Self::Target, F, $A>) -> F
            where
                F: Copy
            {
                impl_fo!(fn read_copy<Self::Target, F, $A>(offset, self))
            }

            #[inline(always)]
            unsafe fn f_read<F>(self, offset: FieldOffset<Self::Target, F, $A>) -> F {
                impl_fo!(fn read<Self::Target, F, $A>(offset, self))
            }
        }
    };
}

macro_rules! impl_ROExtRawMutOps {
    ($A:ident, [$($ptr:tt)*])=>{
        unsafe impl<S> ROExtRawMutOps<$A> for $($ptr)* S {
            #[inline(always)]
            unsafe fn f_write<F>(self, offset: FieldOffset<Self::Target, F, $A>, value: F) {
                impl_fo!(fn write<Self::Target, F, $A>(offset, self, value))
            }

            #[inline(always)]
            unsafe fn f_copy_from<F>(
                self,
                offset: FieldOffset<Self::Target, F, $A>,
                source: *const Self::Target,
            ) {
                impl_fo!(fn copy<Self::Target, F, $A>(offset, source, self))
            }

            #[inline(always)]
            unsafe fn f_copy_from_nonoverlapping<F>(
                self,
                offset: FieldOffset<Self::Target, F, $A>,
                source: *const Self::Target,
            ) {
                impl_fo!(fn copy_nonoverlapping<Self::Target, F, $A>(offset, source, self))
            }

            #[inline(always)]
            unsafe fn f_replace_raw<F>(
                self,
                offset: FieldOffset<Self::Target, F, $A>,
                value: F,
            ) -> F {
                impl_fo!(fn replace<Self::Target, F, $A>(offset, self, value))
            }

            #[inline(always)]
            unsafe fn f_swap_raw<F>(
                self,
                offset: FieldOffset<Self::Target, F, $A>,
                right: *mut Self::Target,
            ) {
                impl_fo!(fn swap<Self::Target, F, $A>(offset, self, right))
            }

            #[inline(always)]
            unsafe fn f_swap_nonoverlapping<F>(
                self,
                offset: FieldOffset<Self::Target, F, $A>,
                right: *mut Self::Target
            ) {
                impl_fo!(fn swap_nonoverlapping<Self::Target, F, $A>(offset, self, right))
            }
        }
    }
}

impl_ROExtRaw! {*const}
impl_ROExtRaw! {*mut}

impl_ROExtRawMut! {*mut}
