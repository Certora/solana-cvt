/// A trait for giving a type a non-deterministic value
pub trait Nondet : Sized {
  fn nondet() -> Self;
}

/// Return a nondet value of type according tot the Nondet trait
pub fn nondet<T: Nondet>() -> T {
    Nondet::nondet()
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

use super::cvt;

nondet_impl! { (), (),  "No nondet value for  unit"}
nondet_impl! { u8, cvt::CVT_nondet_u8(), "Nondet for u8"  }

// XXX: extend with other primitive types