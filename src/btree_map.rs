use std::collections::BTreeMap;
use {
    crate::{
        nondet::{
            Nondet, nondet_impl
        }
    }
};

extern "C" {
    #[allow(improper_ctypes)]
    fn mk_btree_map_unchecked() -> BTreeMap<String, u8>;
}

#[allow(non_snake_case)]
fn CVT_nondet_btree_map_impl() -> BTreeMap<String, u8> {
    unsafe {
        return mk_btree_map_unchecked()
    }
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_btree_map() -> BTreeMap<String, u8> {
    CVT_nondet_btree_map_impl()
}

nondet_impl! {BTreeMap<String, u8>, CVT_nondet_btree_map(), "Nondet for BTreeMap<String, u8>" }
