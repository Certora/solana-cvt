#![allow(unused_macros)]

extern "C" {
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_tag(tag: u64);
    
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_1(tag: u64, x: u64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_2(tag: u64, x: u64, y:u64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_3(tag: u64, x: u64, y:u64, z:u64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_u64_4(tag: u64, v: u64, x: u64, y:u64, z:u64);

    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_1(tag: i64, x: i64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_2(tag: i64, x: i64, y:i64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_3(tag: i64, x: i64, y:i64, z:i64);
    #[allow(dead_code)]
    pub fn cvt_calltrace_print_i64_4(tag: i64, v: i64, x: i64, y:i64, z:i64);
}

#[macro_export]
macro_rules! cvt_cex_print_tag {
    ($tag: expr) => { unsafe {cvt_calltrace_print_tag($tag)}};
}


#[macro_export]
macro_rules! cvt_cex_print_u64 {
    ($tag: expr, $x: expr) => { unsafe {cvt_calltrace_print_u64_1($tag, $x as u64)}};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {cvt_calltrace_print_u64_2($tag, $x as u64,$y as u64)}};
    ($tag: expr, $x: expr, $y: expr, $z: expr) => { unsafe {cvt_calltrace_print_u64_3($tag, $x as u64,$y as u64,$z as u64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr, $z: expr) =>
    { unsafe {cvt_calltrace_print_u64_4($tag, $v as u64, $x as u64,$y as u64,$z as u64)}};
}

#[macro_export]
macro_rules! cvt_cex_print_i64 {
    ($tag: expr, $x: expr) => { unsafe {cvt_calltrace_print_i64_1($tag, $x as i64)}};
    ($tag: expr, $x: expr, $y: expr) => { unsafe {cvt_calltrace_print_i64_2($tag, $x as i64,$y as i64)}};
    ($tag: expr, $x: expr, $y: expr, $z: expr) => { unsafe {cvt_calltrace_print_i64_3($tag, $x as i64,$y as i64,$z as i64)}};
    ($tag: expr, $v: expr, $x: expr, $y: expr, $z: expr) =>
    { unsafe {cvt_calltrace_print_i64_4($tag, $v as i64, $x as i64,$y as i64,$z as i64)}};
}
