use solana_program::{account_info::AccountInfo, program_option::COption, pubkey::Pubkey};

mod rt_decls {
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn CVT_nondet_account_info() -> AccountInfo<'static>;
        pub fn CVT_nondet_pubkey() -> Pubkey;
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    use solana_program::{account_info::AccountInfo, pubkey::Pubkey};
    use solana_sdk::account::Account;
    use std::boxed::Box;

    #[allow(improper_ctypes, improper_ctypes_definitions)]
    #[no_mangle]
    extern "C" fn CVT_nondet_account_info() -> AccountInfo<'static> {
        let owner: &'static mut _ = Box::leak(Box::new(Pubkey::new_unique()));
        let acc = Account::new(1, 2, &owner);
        let pk: &'static mut _ = Box::leak(Box::new(Pubkey::new_unique()));
        let v: &'static mut _ = Box::leak(Box::new((*pk, acc)));
        v.into()
    }
    #[no_mangle]
    pub extern "C" fn CVT_nondet_pubkey() -> Pubkey {
        Pubkey::default()
    }
}

pub fn cvlr_nondet_account_info() -> AccountInfo<'static> {
    unsafe { rt_decls::CVT_nondet_account_info() }
}

pub fn cvlr_nondet_pubkey() -> Pubkey {
    unsafe { rt_decls::CVT_nondet_pubkey() }
}

pub fn cvlr_nondet_option_pubkey() -> Option<Pubkey> {
    cvlr_nondet::nondet_option(|| cvlr_nondet_pubkey())
}

pub fn cvlr_nondet_coption<T, F>(func: F) -> COption<T>
where
    F: FnOnce() -> T,
{
    if cvlr_nondet::nondet::<bool>() {
        COption::Some(func())
    } else {
        COption::None
    }
}

pub fn cvlr_nondet_coption_pubkey() -> COption<Pubkey> {
    cvlr_nondet_coption(|| cvlr_nondet_pubkey())
}