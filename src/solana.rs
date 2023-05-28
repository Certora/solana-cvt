use {
    solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey
    },
    crate::{
        cvt::{
            CVT_nondet_pubkey, CVT_nondet_account_info,
        }, 
        nondet::{
            Nondet, nondet_impl,
        },
    }
};

nondet_impl! {Pubkey, CVT_nondet_pubkey(), "Nondet for Pubkey" }
nondet_impl! {AccountInfo<'static>, CVT_nondet_account_info(), "Nondet for AccountInfo" }
