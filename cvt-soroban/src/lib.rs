#![no_std]
use soroban_sdk::{Address};

mod callable;

pub use callable::Call;

extern "C" {
    fn CVT_SOROBAN_is_auth(address: u64) -> u64;

    // TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
    pub fn CVT_calltrace_print_c_u64 (v: u64);
    pub fn CVT_calltrace_print_c_i64 (v: i64);
    pub fn CVT_calltrace_print_c_u32 (v: u32);
    pub fn CVT_calltrace_print_c_i32 (v: i32);
}

pub fn is_auth(address: Address) -> bool {
    unsafe { CVT_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}

// TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
#[macro_export]
macro_rules! cvt_cex_print_u64 {
    ($x: expr) => { unsafe {CVT_calltrace_print_c_u64($x)}};
}

#[macro_export]
macro_rules! cvt_cex_print_i64 {
    ($x: expr) => { unsafe {CVT_calltrace_print_c_i64($x)}};
}

#[macro_export]
macro_rules! cvt_cex_print_u32 {
    ($x: expr) => { unsafe {CVT_calltrace_print_c_u32($x)}};
}

#[macro_export]
macro_rules! cvt_cex_print_i32 {
    ($x: expr) => { unsafe {CVT_calltrace_print_c_i32($x)}};
}
