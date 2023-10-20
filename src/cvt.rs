use super::cvt_stubs;

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_assume(c: bool){
    cvt_stubs::CVT_assume_impl(c)
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_assert(c: bool){
    cvt_stubs::CVT_assert_impl(c)
}

#[inline(always)]
#[allow(non_snake_case)]
pub fn CVT_satisfy(c: bool){
    CVT_assume(c);
    CVT_assert(false);
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_u8() ->  u8 {
    cvt_stubs::CVT_nondet_u8_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_u16() ->  u16 {
    cvt_stubs::CVT_nondet_u16_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_u32() ->  u32 {
    cvt_stubs::CVT_nondet_u32_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_u64() ->  u64 {
    cvt_stubs::CVT_nondet_u64_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_usize() ->  usize { cvt_stubs::CVT_nondet_usize_impl() }

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_i8() ->  i8 {
    cvt_stubs::CVT_nondet_i8_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_i16() ->  i16 {
    cvt_stubs::CVT_nondet_i16_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_i32() -> i32 {
    cvt_stubs::CVT_nondet_i32_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_i64() ->  i64 {
    cvt_stubs::CVT_nondet_i64_impl()
}

extern "C" {
    // Rust: we cannot use type parameters on foreign items
    // fn CVT_nondet_pointer_c<T: Nondet>() -> *mut T;
    pub fn CVT_nondet_pointer_usize() -> *mut usize;
}

#[macro_export]
macro_rules! cvt_uninterpreted_usize {
    ($fname:ident, $gname: ident) => {
        static mut $gname: *mut usize = std::ptr::null_mut();
        #[allow(non_snake_case)]
        pub fn $fname() ->  usize {
            unsafe {
                if $gname.is_null() {
                    $gname = cvt::CVT_nondet_pointer_usize()
                }
                *$gname
             }
        }
    };
}

#[inline(never)]
#[allow(non_snake_case)]
// Return an array of 32 bytes initialized non-deterministically
pub fn CVT_nondet_array_of_32_bytes() -> [u8; 32] {
    cvt_stubs::CVT_nondet_array_of_32_bytes_impl()
}

// We redefine these macros to avoid including error conversion/formatting code
#[macro_export]
macro_rules! require {
    ($invariant:expr, $error:tt $(,)?) => {
        cvt::CVT_assume($invariant);
    };
    ($invariant:expr, $error:expr $(,)?) => {
        cvt::CVT_assume($invariant);
    };
}

#[macro_export]
macro_rules! require_gte {
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        cvt::CVT_assume($value1 >= $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        cvt::CVT_assume($value1 >= $value2);
    };
}

#[macro_export]
macro_rules! assert {
        ($cond:expr) => {{ cvt::CVT_assert($cond)}};
}