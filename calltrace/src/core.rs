#![allow(unused_macros)]

extern "C" {
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_tag(tag: &str);
    
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_1(tag: &str, x: u64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_2(tag: &str, x: u64, y:u64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_3(tag: &str, x: u64, y:u64, z:u64);

    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_1(tag: &str, x: i64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_2(tag: &str, x: i64, y:i64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_3(tag: &str, x: i64, y:i64, z:i64);
}


#[macro_export]
macro_rules! cvt_cex_print_tag {
    ($tag: expr) => { unsafe {::calltrace::cvt_calltrace_print_tag(stringify!($tag))}};
}


#[macro_export]
macro_rules! cvt_cex_print_u64 {
    ($tag: expr, $x: expr) => { unsafe {::calltrace::cvt_calltrace_print_u64_1(stringify!($tag), $x as u64)}};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {::calltrace::cvt_calltrace_print_u64_2(stringify!($tag), $x as u64,$y as u64)}};
    ($tag: expr, $x: expr, $y: expr, $z: expr) => { unsafe {::calltrace::cvt_calltrace_print_u64_3(stringify!($tag), $x as u64,$y as u64,$z as u64)}};
}

#[macro_export]
macro_rules! cvt_cex_print_i64 {
    ($tag: expr, $x: expr) => { unsafe {::calltrace::cvt_calltrace_print_i64_1(stringify!($tag), $x as i64)}};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {::calltrace::cvt_calltrace_print_i64_2(stringify!($tag), $x as i64,$y as i64)}};
    ($tag: expr, $x: expr, $y: expr, $z: expr) => { unsafe {::calltrace::cvt_calltrace_print_i64_3(stringify!($tag), $x as i64,$y as i64,$z as i64)}};
}
