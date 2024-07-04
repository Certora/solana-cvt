use crate::Nondet;

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
use stubs::solana_stubs;

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_account_info() -> AccountInfo<'static> {
    solana_stubs::CVT_nondet_account_info_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey() -> Pubkey {
    solana_stubs::CVT_nondet_pubkey_impl()
}

// E0117: need to implenet Nondet for Pubkey and AccountInfo here instead of solana/src/lib.rs
// because we can only implement a trait for an arbitrary type in the crate where the trait is defined
crate::nondet_impl! {Pubkey, CVT_nondet_pubkey(), "Nondet for Pubkey" }
crate::nondet_impl! {AccountInfo<'static>, CVT_nondet_account_info(), "Nondet for AccountInfo" }
