extern "C" {
    // External assume and assert
    fn CERTORA_assume_c(_c: bool);
    fn CERTORA_assert_c(_c: bool);
    fn CERTORA_satisfy_c(_c: bool);
}

extern "C" {
    // Definition of external functions that represent getting arbitrary values
    fn CERTORA_nondet_u8_c() -> u8;
    fn CERTORA_nondet_u16_c() -> u16;
    fn CERTORA_nondet_u32_c() -> u32;
    fn CERTORA_nondet_u64_c() -> u64;
    fn CERTORA_nondet_usize_c() -> usize;

    fn CERTORA_nondet_i8_c() -> i8;
    fn CERTORA_nondet_i16_c() -> i16;
    fn CERTORA_nondet_i32_c() -> i32;
    fn CERTORA_nondet_i64_c() -> i64;
}

#[allow(non_snake_case)]
pub fn CERTORA_assume_impl(c: bool) {
    unsafe {
        CERTORA_assume_c(c);
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_assert_impl(c: bool) {
    unsafe {
        CERTORA_assert_c(c);
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_satisfy_impl(c: bool) {
    unsafe {
        CERTORA_satisfy_c(c);
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u8_impl() -> u8 {
    unsafe {
        return CERTORA_nondet_u8_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u16_impl() -> u16 {
    unsafe {
        return CERTORA_nondet_u16_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u32_impl() -> u32 {
    unsafe {
        return CERTORA_nondet_u32_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u64_impl() -> u64 {
    unsafe {
        return CERTORA_nondet_u64_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_usize_impl() -> usize {
    unsafe {
        return CERTORA_nondet_usize_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i8_impl() -> i8 {
    unsafe {
        return CERTORA_nondet_i8_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i16_impl() -> i16 {
    unsafe {
        return CERTORA_nondet_i16_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i32_impl() -> i32 {
    unsafe {
        return CERTORA_nondet_i32_c();
    }
}
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i64_impl() -> i64 {
    unsafe {
        return CERTORA_nondet_i64_c();
    }
}

macro_rules! CERTORA_nondet_array_of_bytes {
    ($name_impl:ident, $name_c: ident, $size:expr) => {
        #[allow(improper_ctypes)]
        #[allow(non_snake_case)]
        extern "C" {
            fn $name_c() -> [u8; $size];
        }
        #[allow(non_snake_case)]
        pub fn $name_impl() -> [u8; $size] {
            unsafe {
                return $name_c();
            }
        }
    };
}

CERTORA_nondet_array_of_bytes!(
    CERTORA_nondet_array_of_32_bytes_impl,
    CERTORA_nondet_array_of_32_bytes_c,
    32
);

#[cfg(feature = "impls")]
mod impls {
    //! Default implementations of external functions
    //!
    //! These functions are not expected to be called. They are here only to
    //! allow compining test modules that require all external functions to be
    //! resolved

    #[allow(non_snake_case)]
    #[no_mangle]
    extern "C" fn CERTORA_assume_c(_c: bool) {
        //assert!(_c, "Assumption was not satisfied");
    }

    #[allow(non_snake_case)]
    #[no_mangle]
    extern "C" fn CERTORA_assert_c(_c: bool) {
        assert!(_c);
    }

    macro_rules! impl_nd {
        ($name:ident, $ty:ident) => {
            #[allow(non_snake_case)]
            #[no_mangle]
            extern "C" fn $name() -> $ty {
                Default::default()
            }
        };
    }

    impl_nd!(CERTORA_nondet_u8_c, u8);
    impl_nd!(CERTORA_nondet_u64_c, u64);
    impl_nd!(CERTORA_nondet_i64_c, i64);
}
