#![no_std]
use soroban_sdk::{Address};

mod callable;

pub use callable::Call;

extern "C" {
    fn CVT_SOROBAN_is_auth(address: u64) -> u64;
}

pub fn is_auth(address: Address) -> bool {
    unsafe { CVT_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}
