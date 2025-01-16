use crate::Nondet;

mod rt_decls {
    #[allow(improper_ctypes)]
    extern "C" {
        // Definition of external functions that represent getting arbitrary values
        pub fn CVT_nondet_u8_c() -> u8;
        pub fn CVT_nondet_u16_c() -> u16;
        pub fn CVT_nondet_u32_c() -> u32;
        pub fn CVT_nondet_u64_c() -> u64;
        pub fn CVT_nondet_small_u128_c() -> u128;
        pub fn CVT_nondet_u128_c() -> u128;
        pub fn CVT_nondet_usize_c() -> usize;

        pub fn CVT_nondet_i8_c() -> i8;
        pub fn CVT_nondet_i16_c() -> i16;
        pub fn CVT_nondet_i32_c() -> i32;
        pub fn CVT_nondet_i64_c() -> i64;
        pub fn CVT_nondet_small_i128_c() -> i128;
        pub fn CVT_nondet_i128_c() -> i128;
    }
}

#[cfg(feature = "rt")]
#[allow(improper_ctypes_definitions)]
mod rt_impls {
    macro_rules! impl_rt_fn {
        // 1st argument is ignored
        ($name:ident, $c_name:ident, $ty:ident) => {
            #[no_mangle]
            pub extern "C" fn $c_name() -> $ty {
                $ty::default()
            }
        };
    }
    impl_rt_fn!(cvlr_nondet_u8, CVT_nondet_u8_c, u8);
    impl_rt_fn!(cvlr_nondet_u16, CVT_nondet_u16_c, u16);
    impl_rt_fn!(cvlr_nondet_u32, CVT_nondet_u32_c, u32);
    impl_rt_fn!(cvlr_nondet_u64, CVT_nondet_u64_c, u64);
    impl_rt_fn!(cvlr_nondet_u128, CVT_nondet_u128_c, u128);
    impl_rt_fn!(cvlr_nondet_usize, CVT_nondet_usize_c, usize);

    impl_rt_fn!(cvlr_nondet_i8, CVT_nondet_i8_c, i8);
    impl_rt_fn!(cvlr_nondet_i16, CVT_nondet_i16_c, i16);
    impl_rt_fn!(cvlr_nondet_i32, CVT_nondet_i32_c, i32);
    impl_rt_fn!(cvlr_nondet_i64, CVT_nondet_i64_c, i64);
    impl_rt_fn!(cvlr_nondet_i128, CVT_nondet_i128_c, i128);

    impl_rt_fn!(cvlr_nondet_small_u128, CVT_nondet_small_u128_c, u128);
    impl_rt_fn!(cvlr_nondet_small_i128, CVT_nondet_small_i128_c, i128);
}

use rt_decls::*;

macro_rules! impl_checked_fn {
    ($name:ident, $c_name:ident, $ty:ident) => {
        #[inline(always)]
        pub fn $name() -> $ty {
            unsafe { $c_name() }
        }
    };
}

impl_checked_fn!(cvlr_nondet_u8, CVT_nondet_u8_c, u8);
impl_checked_fn!(cvlr_nondet_u16, CVT_nondet_u16_c, u16);
impl_checked_fn!(cvlr_nondet_u32, CVT_nondet_u32_c, u32);
impl_checked_fn!(cvlr_nondet_u64, CVT_nondet_u64_c, u64);
impl_checked_fn!(cvlr_nondet_u128, CVT_nondet_u128_c, u128);
impl_checked_fn!(cvlr_nondet_usize, CVT_nondet_usize_c, usize);

impl_checked_fn!(cvlr_nondet_i8, CVT_nondet_i8_c, i8);
impl_checked_fn!(cvlr_nondet_i16, CVT_nondet_i16_c, i16);
impl_checked_fn!(cvlr_nondet_i32, CVT_nondet_i32_c, i32);
impl_checked_fn!(cvlr_nondet_i64, CVT_nondet_i64_c, i64);
impl_checked_fn!(cvlr_nondet_i128, CVT_nondet_i128_c, i128);

impl_checked_fn!(cvlr_nondet_small_u128, CVT_nondet_small_u128_c, u128);
impl_checked_fn!(cvlr_nondet_small_i128, CVT_nondet_small_i128_c, i128);

crate::nondet_impl! { (), (),  "No nondet value for  unit" }
crate::nondet_impl! { bool, cvlr_nondet_u64() > 0, "Nondet for bool"}
crate::nondet_impl! { u8, cvlr_nondet_u8(), "Nondet for u8" }
crate::nondet_impl! { i8, cvlr_nondet_i8(), "Nondet for i8" }
crate::nondet_impl! { u16, cvlr_nondet_u16(), "Nondet for u16" }
crate::nondet_impl! { i16, cvlr_nondet_i16(), "Nondet for i16" }
crate::nondet_impl! { u32, cvlr_nondet_u32(), "Nondet for u32" }
crate::nondet_impl! { i32, cvlr_nondet_i32(), "Nondet for i32" }
crate::nondet_impl! { u64, cvlr_nondet_u64(), "Nondet for u64" }
crate::nondet_impl! { i64, cvlr_nondet_i64(), "Nondet for i64" }
crate::nondet_impl! { u128, cvlr_nondet_u128(), "Nondet for u128" }
crate::nondet_impl! { i128, cvlr_nondet_i128(), "Nondet for i128" }

crate::nondet_impl! { usize, cvlr_nondet_usize(), "Nondet for usize" }
