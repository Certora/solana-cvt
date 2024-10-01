use soroban_sdk::{Address, Env, TryFromVal, Val, String, Bytes};

use crate::{Nondet};

impl Nondet for u128 {
    fn nondet() -> Self {
        let u1 = u64::nondet();
        let u2 = u64::nondet();
        (u1 as u128) << 64 | u2 as u128
    }
}

impl Nondet for i128 {
    fn nondet() -> i128 { u128::nondet() as i128 }
}

impl Nondet for Address {
    fn nondet() -> Self {
        let v = u64::nondet();
        let val = Val::from_payload((v << 8) | 77);
        Address::try_from_val(&Env::default(), &val).unwrap()
    }
}

impl Nondet for String {
    fn nondet() -> Self {
        let nondet_u8 = u8::nondet();
        return String::from_bytes(&Env::default(), &[nondet_u8]);
    }
}

impl Nondet for Bytes {
    fn nondet() -> Self {
        let nondet_u8 = u8::nondet();
        return Bytes::from_slice(&Env::default(), &[nondet_u8]);
    }
}