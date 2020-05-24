#![allow(dead_code)]

#[repr(C, packed)]
pub(crate) struct Packed<T>(T);

// Emulates MaybeUninit before Rust 1.36,when it was stabilized.
#[repr(u8)]
pub(crate) enum UnalignedMaybeUninit<T> {
    Init(Packed<T>),
    Uninit,
}

impl<T> UnalignedMaybeUninit<T> {
    #[inline(always)]
    pub const fn uninit() -> Self {
        UnalignedMaybeUninit::Uninit
    }
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        unsafe {
            // The offset is 1 because `Packed` always has an alignment of 1,
            // causing the payload for the Init variant to be put right after
            // the `u8` discriminant.
            (self as *mut Self as *mut u8).offset(1) as *mut T
        }
    }
}

macro_rules! size_assertions {
    ( $($type:ty),* $(,)? ) => (
        fn _size_assertions(){
            use core::mem::size_of;
            $(
                let _: [(); size_of::<UnalignedMaybeUninit<$type>>() ] =
                    [(); size_of::<$type>() + 1 ];
            )*
        }
    )
}

size_assertions! {
    &'static str,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_mymaybeuninit() {
        unsafe {
            let mut space = UnalignedMaybeUninit::<&'static str>::uninit();
            let ptr = space.as_mut_ptr();
            ptr.write_unaligned("hello");
            assert_eq!(ptr.read_unaligned(), "hello");
        }
    }
}
