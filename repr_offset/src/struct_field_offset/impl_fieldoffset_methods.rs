// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `raw_get`
macro_rules! get_ptr_method {
    ($self:expr, $base:expr, $S:ty, $F:ty) => {{
        #[cfg(feature = "testing")]
        let _: *const $S = $base;

        ($base as *const $S as *const u8).offset($self.offset as isize) as *const $F
    }};
}

// Defined this macro to reduce the amount of instructions in debug builds
// caused by delegating to `raw_get_mut`
macro_rules! get_mut_ptr_method {
    ($self:expr, $base:expr, $S:ty, $F:ty) => {{
        #[cfg(feature = "testing")]
        let _: *mut $S = $base;

        ($base as *mut $S as *mut u8).offset($self.offset as isize) as *mut $F
    }};
}

macro_rules! replace_unaligned {
    ($self:expr, $base:expr, $value:expr, $S:ty, $F:ty) => {{
        let ptr = get_mut_ptr_method!($self, $base, $S, $F);
        let ret = ptr.read_unaligned();
        ptr.write_unaligned($value);
        ret
    }};
}

macro_rules! unaligned_swap {
    ($self:expr, $left:expr, $right:expr, $left_to_right:expr, $S:ty, $F:ty) => {{
        // This function can definitely be optimized.
        let mut tmp = core::mem::MaybeUninit::<$F>::uninit();
        let tmp = tmp.as_mut_ptr() as *mut u8;

        let left = get_mut_ptr_method!($self, $left, $S, $F) as *mut u8;
        let right = get_mut_ptr_method!($self, $right, $S, $F) as *mut u8;
        core::ptr::copy_nonoverlapping(left, tmp, crate::utils::Mem::<$F>::SIZE);
        $left_to_right(right, left, crate::utils::Mem::<$F>::SIZE);
        core::ptr::copy_nonoverlapping(tmp, right, crate::utils::Mem::<$F>::SIZE);
    }};
}

macro_rules! impl_fo {
    (fn get<$S:ty, $F:ty, Aligned>($self:expr, $base:expr)) => {
        &*get_ptr_method!($self, $base, $S, $F)
    };
    (fn get_mut<$S:ty, $F:ty, Aligned>($self:expr, $base:expr)) => {
        &mut *get_mut_ptr_method!($self, $base, $S, $F)
    };
    (fn get_ptr<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        get_ptr_method!($self, $base, $S, $F)
    };
    (fn get_mut_ptr<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        get_mut_ptr_method!($self, $base, $S, $F)
    };
    (fn raw_get<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        get_ptr_method!($self, $base, $S, $F)
    };
    (fn raw_get_mut<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        get_mut_ptr_method!($self, $base, $S, $F)
    };
    (fn get_copy<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        if_aligned! {
            $A {
                *get_ptr_method!($self, $base, $S, $F)
            } else {
                get_ptr_method!($self, $base, $S, $F).read_unaligned()
            }
        }
    };
    (fn read_copy<$S:ty, $F:ty, $A:ident>($self:expr, $base:expr)) => {
        if_aligned! {
            $A {
                *get_ptr_method!($self, $base, $S, $F)
            } else {
                get_ptr_method!($self, $base, $S, $F).read_unaligned()
            }
        }
    };
    (fn read<$S:ty, $F:ty, $A:ident>($self:expr, $source:ident)) => {
        if_aligned! {
            $A {
                get_ptr_method!($self, $source, $S, $F).read()
            } else {
                get_ptr_method!($self, $source, $S, $F).read_unaligned()
            }
        }
    };
    (fn write<$S:ty, $F:ty, $A:ident>($self:expr, $dst:ident, $value:ident)) => {
        if_aligned! {
            $A {
                get_mut_ptr_method!($self, $dst, $S, $F).write($value)
            } else {
                get_mut_ptr_method!($self, $dst, $S, $F).write_unaligned($value)
            }
        }
    };
    (fn copy<$S:ty, $F:ty, $A:ident>($self:expr, $source:ident, $dst:ident)) => {
        if_aligned! {
            $A {
                core::ptr::copy(
                    get_ptr_method!($self, $source, $S, $F),
                    get_mut_ptr_method!($self, $dst, $S, $F),
                    1,
                )
            } else {
                core::ptr::copy(
                    get_ptr_method!($self, $source, $S, $F) as *const u8,
                    get_mut_ptr_method!($self, $dst, $S, $F) as *mut u8,
                    crate::utils::Mem::<F>::SIZE,
                )
            }
        }
    };
    (fn copy_nonoverlapping<$S:ty, $F:ty, $A:ident>($self:expr, $source:ident, $dst:ident)) => {
        if_aligned! {
            $A {
                core::ptr::copy_nonoverlapping(
                    get_ptr_method!($self, $source, $S, $F),
                    get_mut_ptr_method!($self, $dst, $S, $F),
                    1,
                )
            } else {
                core::ptr::copy_nonoverlapping(
                    get_ptr_method!($self, $source, $S, $F) as *const u8,
                    get_mut_ptr_method!($self, $dst, $S, $F) as *mut u8,
                    crate::utils::Mem::<F>::SIZE,
                )
            }
        }
    };
    (fn replace<$S:ty, $F:ty, $A:ident>($self:expr, $dst:ident, $value:ident)) => {
        if_aligned! {
            $A {
                core::ptr::replace(get_mut_ptr_method!($self, $dst, $S, $F), $value)
            } else {
                replace_unaligned!($self, $dst, $value, $S, $F)
            }
        }
    };
    (fn replace_mut<$S:ty, $F:ty, $A:ident>($self:expr, $dst:ident, $value:ident)) => {
        if_aligned! {
            $A {
                core::mem::replace(&mut *get_mut_ptr_method!($self, $dst, $S, $F), $value)
            } else {
                replace_unaligned!($self, $dst, $value, $S, $F)
            }
        }
    };
    (fn swap<$S:ty, $F:ty, $A:ident>($self:expr, $l:ident, $r:ident)) => {
        if_aligned! {
            $A {
                core::ptr::swap::<F>(
                    get_mut_ptr_method!($self, $l, $S, $F),
                    get_mut_ptr_method!($self, $r, $S, $F),
                )
            } else {
                unaligned_swap!($self, $l, $r, core::ptr::copy, $S, $F)
            }
        }
    };
    (fn swap_nonoverlapping<$S:ty, $F:ty, $A:ident>($self:expr, $l:ident, $r:ident)) => {
        if_aligned! {
            $A {
                core::ptr::swap_nonoverlapping::<F>(
                    get_mut_ptr_method!($self, $l, $S, $F),
                    get_mut_ptr_method!($self, $r, $S, $F),
                    1,
                )
            } else {
                unaligned_swap!($self, $l, $r, core::ptr::copy_nonoverlapping, $S, $F)
            }
        }
    };
    (fn swap_mut<$S:ty, $F:ty, $A:ident>($self:expr, $l:ident, $r:ident)) => {
        if_aligned! {
            $A {
                core::mem::swap(
                    &mut *get_mut_ptr_method!($self, $l, $S, $F),
                    &mut *get_mut_ptr_method!($self, $r, $S, $F),
                )
            } else {{
                // This function could probably be optimized.
                let l = get_mut_ptr_method!($self, $l, $S, $F);
                let r = get_mut_ptr_method!($self, $r, $S, $F);
                let tmp = l.read_unaligned();
                l.write_unaligned(r.read_unaligned());
                r.write_unaligned(tmp);
            }}
        }
    };
}

macro_rules! if_aligned {
    (Aligned {$($then:tt)*} else {$($else:tt)*}) => {
        $($then)*
    };
    (Unaligned {$($then:tt)*} else {$($else:tt)*}) => {
        $($else)*
    };
}
