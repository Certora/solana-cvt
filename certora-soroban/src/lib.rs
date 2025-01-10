#![no_std]
use soroban_sdk::{Address};

mod callable;

pub use callable::Call;

extern "C" {
    fn CERTORA_SOROBAN_is_auth(address: u64) -> u64;

    // TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
    pub fn CERTORA_calltrace_print_c_u64 (s: &str, v: u64);
    pub fn CERTORA_calltrace_print_c_i64 (s: &str, v: i64);
    pub fn CERTORA_calltrace_print_c_u32 (s: &str, v: u32);
    pub fn CERTORA_calltrace_print_c_i32 (s: &str, v: i32);
}

pub fn is_auth(address: Address) -> bool {
    unsafe { CERTORA_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}

// TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
#[macro_export]
macro_rules! certora_print_u64 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_u64($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_i64 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_i64($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_u32 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_u32($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_i32 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_i32($vnm, $x)}};
}
