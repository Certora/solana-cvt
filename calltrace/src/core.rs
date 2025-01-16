pub mod rt_decls {
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn CVT_calltrace_print_tag(tag: &str);

        pub fn CVT_calltrace_print_u64_1(tag: &str, x: u64);
        pub fn CVT_calltrace_print_u64_2(tag: &str, x: u64, y: u64);
        pub fn CVT_calltrace_print_u64_3(tag: &str, x: u64, y: u64, z: u64);

        pub fn CVT_calltrace_print_i64_1(tag: &str, x: i64);
        pub fn CVT_calltrace_print_i64_2(tag: &str, x: i64, y: i64);
        pub fn CVT_calltrace_print_i64_3(tag: &str, x: i64, y: i64, z: i64);
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_tag(_tag: &str) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_1(_tag: &str, _x: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_2(_tag: &str, _x: u64, _y: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_u64_3(_tag: &str, _x: u64, _y: u64, _z: u64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_1(_tag: &str, _x: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_2(_tag: &str, _x: i64, _y: i64) {}
    #[no_mangle]
    pub extern "C" fn CVT_calltrace_print_i64_3(_tag: &str, _x: i64, _y: i64, _z: i64) {}
}
pub use rt_decls::*;

pub struct CvlrLogger;

impl CvlrLogger {
    pub fn new() -> Self {
        Self {}
    }
    pub fn log(&mut self, v: &str) {
        unsafe {
            CVT_calltrace_print_tag(v);
        }
    }

    pub fn log_u64(&mut self, t: &str, v: u64) {
        unsafe {
            CVT_calltrace_print_u64_1(t, v);
        }
    }
    pub fn log_u64_2(&mut self, t: &str, v0: u64, v1: u64) {
        unsafe {
            CVT_calltrace_print_u64_2(t, v0, v1);
        }
    }
    pub fn log_u64_3(&mut self, t: &str, v0: u64, v1: u64, v2: u64) {
        unsafe {
            CVT_calltrace_print_u64_3(t, v0, v1, v2);
        }
    }

    pub fn log_i64(&mut self, t: &str, v: i64) {
        unsafe {
            CVT_calltrace_print_i64_1(t, v);
        }
    }
}



#[inline(always)]
pub fn log(v: &str) {
    let mut logger = CvlrLogger::new();
    logger.log(v);
}

#[inline(always)]
pub fn log_u64(t: &str, v: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_u64(t, v);
}

#[inline(always)]
pub fn log_i64(t: &str, v: u64) {
    let mut logger = CvlrLogger::new();
    logger.log_u64(t, v);
}

#[macro_export]
macro_rules! cvt_cex_print_tag {
    ($tag: expr) => {
        ::calltrace::log(stringify($tag))
    };
}

#[macro_export]
macro_rules! cvt_cex_print_u64 {
    ($tag: expr $(,)?) => {};
    ($tag: expr, $x: expr) => { ::calltrace::log_u64(stringify!($tag), $x as u64) };
    ($tag: expr, $x: expr, $y: expr) => { unsafe {::calltrace::CVT_calltrace_print_u64_2(stringify!($tag), $x as u64,$y as u64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr $(,$z:expr)*) =>
    {
	unsafe {::calltrace::CVT_calltrace_print_u64_3(stringify!($tag), $v as u64, $x as u64,$y as u64)};
	cvt_cex_print_u64!($tag $(,$z)*);
	() // it needs to return an expression
    };
}

#[macro_export]
macro_rules! cvt_cex_print_i64 {
    ($tag: expr $(,)?) => {};
    ($tag: expr, $x: expr) => { ::calltrace::log_i64(stringify!($tag), $x as i64)};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {::calltrace::CVT_calltrace_print_i64_2(stringify!($tag), $x as i64,$y as i64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr $(,$z:expr)*) =>
    {
	unsafe {::calltrace::CVT_calltrace_print_i64_3(stringify!($tag), $v as i64, $x as i64,$y as i64)};
	cvt_cex_print_i64!($tag $(,$z)*);
	() // it needs to return an expression
    };
}
