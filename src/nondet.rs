/// A trait for giving a type a non-deterministic value
pub trait Nondet: Sized {
    fn nondet() -> Self;

    /// i32::nondet_with(|x| x > 0)
    fn nondet_with<F>(func: F) -> Self
    where
        F: FnOnce(&Self) -> bool,
    {
        let val = Self::nondet();
        crate::CVT_assume(func(&val));
        val
    }
}

/// Return a nondet value of type according tot the Nondet trait
pub fn nondet<T: Nondet>() -> T {
    Nondet::nondet()
}

pub fn nondet_with<T: Nondet, F>(func: F) -> T
where
    F: FnOnce(&T) -> bool,
{
    Nondet::nondet_with(func)
}

macro_rules! nondet_impl {
    ($t:ty, $v:expr, $doc:tt) => {
        impl Nondet for $t {
            #[inline]
            #[doc = $doc]
            fn nondet() -> $t {
                $v
            }
        }
    };
}
pub(crate) use nondet_impl;

use super::cvt;

nondet_impl! { (), (),  "No nondet value for  unit" }
nondet_impl! { u8, cvt::CVT_nondet_u8(), "Nondet for u8" }
nondet_impl! { i8, cvt::CVT_nondet_i8(), "Nondet for i8" }
nondet_impl! { u16, cvt::CVT_nondet_u16(), "Nondet for u16" }
nondet_impl! { i16, cvt::CVT_nondet_i16(), "Nondet for i16" }
nondet_impl! { u32, cvt::CVT_nondet_u32(), "Nondet for u32" }
nondet_impl! { i32, cvt::CVT_nondet_i32(), "Nondet for i32" }
nondet_impl! { u64, cvt::CVT_nondet_u64(), "Nondet for u64" }
nondet_impl! { i64, cvt::CVT_nondet_i64(), "Nondet for i64" }
nondet_impl! { usize, cvt::CVT_nondet_usize(), "Nondet for usize" }

nondet_impl! { [u8; 32], cvt::CVT_nondet_array_of_32_bytes(), "Nondet for 32-byte array" }
