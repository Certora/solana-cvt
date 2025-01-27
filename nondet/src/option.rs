use crate::{nondet, Nondet};

impl<T: Nondet> Nondet for Option<T> {
    #[inline]
    fn nondet() -> Option<T> {
        if nondet::<bool>() {
            Some(nondet::<T>())
        } else {
            None
        }
    }
}
