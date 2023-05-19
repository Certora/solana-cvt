use super::cvt_stubs;

use std::collections::BTreeMap;
use {
    solana_program:: {
        account_info::{AccountInfo},
        pubkey::Pubkey
    }
};

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

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_u8() ->  u8 {
    cvt_stubs::CVT_nondet_u8_impl()
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
pub fn CVT_nondet_usize() ->  usize {
    cvt_stubs::CVT_nondet_usize_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_i8() ->  i8 {
    cvt_stubs::CVT_nondet_i8_impl()
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


#[inline(never)]
#[allow(non_snake_case)]
// Return an arbitrary usize but always the same one
pub fn CVT_uninterpreted_usize() ->  usize { cvt_stubs::CVT_uninterpreted_usize_impl()}

#[inline(never)]
#[allow(non_snake_case)]
// TODO: use macro to allow arrays of different sizes without having a separate function per size
pub fn CVT_nondet_array_of_32_bytes() -> [u8; 32] {
    cvt_stubs::CVT_nondet_pubkey_impl().to_bytes()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_btree_map() -> BTreeMap<String, u8> {
    cvt_stubs::CVT_nondet_btree_map_impl()
}

/* Stubs for solana_program */

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_account_info() -> AccountInfo<'static> {
    cvt_stubs::CVT_nondet_account_info_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey() -> Pubkey {
    cvt_stubs::CVT_nondet_pubkey_impl()
}
