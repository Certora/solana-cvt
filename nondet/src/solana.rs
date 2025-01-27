#![allow(unused_doc_comments)]

use crate::{nondet, Nondet};

use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

impl<T: Nondet> Nondet for solana_program::program_option::COption<T> {
    fn nondet() -> Self {
        if nondet::<bool>() {
            Self::None
        } else {
            Self::Some(nondet::<T>())
        }
    }
}

mod rt_decls {
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn mk_account_info_unchecked() -> AccountInfo<'static>;
        pub fn mk_pubkey_unchecked() -> Pubkey;
    }
}

#[cfg(feature = "std")]
#[cfg(feature = "rt")]
mod rt_impls {
    use std::boxed::Box;
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    use solana_sdk::account::Account;


    #[allow(improper_ctypes,improper_ctypes_definitions)]
    #[no_mangle]
    extern "C" fn mk_account_info_unchecked() -> AccountInfo<'static> {
        let owner : &'static mut _ = Box::leak(Box::new(Pubkey::new_unique()));
        let acc = Account::new(1, 2, &owner);
        let pk : &'static mut _ = Box::leak(Box::new(Pubkey::new_unique()));
        let v : &'static mut _ = Box::leak(Box::new((*pk, acc)));
        v.into()

    }
    #[no_mangle]
    pub extern "C" fn mk_pubkey_unchecked() -> Pubkey {
        Pubkey::default()
    }

}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_account_info() -> AccountInfo<'static> {
    unsafe { rt_decls::mk_account_info_unchecked() }
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey() -> Pubkey {
    unsafe { rt_decls::mk_pubkey_unchecked() }
}

crate::nondet_impl! { Pubkey, CVT_nondet_pubkey(), "Nondet for Pubkey" }
crate::nondet_impl! { AccountInfo<'static>, CVT_nondet_account_info(), "Nondet for AccountInfo" }

