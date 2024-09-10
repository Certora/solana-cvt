
use soroban_sdk::{Address, Env, TryFromVal, Val};

use nondet::{*};

// impl Nondet for Val {
//     fn nondet() -> Self {
//         let v: u64 = u64::nondet();
//         Val::try_from_val(&Env::default(), &(&(v << 8) | (77 as u64))).unwrap()
//     }
    
// }

// impl Nondet for Address {
//     fn nondet() -> Self {
//         Address::try_from_val(&Env::default(), &Val::nondet()).unwrap()
//     }
// }

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_address() -> Address {
    CVT_nondet_account_info_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CVT_nondet_val() -> Val {
    CVT_nondet_val_impl()
}

#[allow(non_snake_case)]
pub fn CVT_nondet_address_impl() -> Address {
    unsafe {
        return mk_nondet_address();
    }
}

#[allow(non_snake_case)]
pub fn CVT_nondet_val_impl() -> Address {
    unsafe {
        return mk_nondet_val();
    }
}

#[allow(improper_ctypes,improper_ctypes_definitions)]
#[no_mangle]
extern "C" fn mk_nondet_val() -> Val {
    let v: u64 = u64::nondet();
    Val::try_from_val(&Env::default(), &(&(v << 8) | (77 as u64))).unwrap()
}

#[allow(improper_ctypes,improper_ctypes_definitions)]
#[no_mangle]
extern "C" fn mk_nondet_address() -> Address {
    Address::try_from_val(&Env::default(), &mk_nondet_val()).unwrap()
}