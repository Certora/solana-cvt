mod rt_decls {
    extern "C" {
        pub fn CVT_assume(_c: bool);
        pub fn CVT_assert(_c: bool);
        pub fn CVT_satisfy(_c: bool);
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CTV_assume(c: bool) {
        if !c {
            panic!()
        }
    }

    #[no_mangle]
    pub extern "C" fn CVT_assert(c: bool) {
        assert!(c);
    }

    #[no_mangle]
    pub extern "C" fn CVT_satisfy(c: bool) {
        assert!(c);
    }
}

use rt_decls::*;

#[inline(always)]
pub fn cvlr_assert_checked(c: bool) {
    unsafe {
        CVT_assert(c);
    }
}

#[inline(always)]
pub fn cvlr_assume_checked(c: bool) {
    unsafe {
        CVT_assume(c);
    }
}

#[inline(always)]
pub fn cvlr_satisfy_checked(c: bool) {
    unsafe {
        CVT_satisfy(c);
    }
}

#[macro_export]
macro_rules! cvlr_assert {
    ($cond: expr $(,)?) => {
        $crate::cvlr_assert_checked($cond)
    };
}

#[macro_export]
macro_rules! cvlr_assume {
    ($cond: expr $(,)?) => {
        $crate::cvlr_assume_checked($cond)
    };
}

#[macro_export]
macro_rules! cvlr_satisfy {
    ($cond: expr $(,)?) => {
        $crate::cvlr_satisfy_checked($cond)
    };
}

#[cfg(feature = "vacuity")]
#[macro_export]
macro_rules! cvlr_vacuity_check {
    () => { $crate::cvlr_assert!(false) };
}

#[cfg(not(feature = "vacuity"))]
#[macro_export]
macro_rules! cvlr_vacuity_check {
    () => { $crate::cvlr_assert!(true) };
}

