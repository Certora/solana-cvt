
#![no_std]
use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, Map, Vec};

extern "C" {
    fn CVT_SOROBAN_is_auth(address: u64) -> u64;
}

fn is_auth(address: Address) -> bool {
    unsafe { CVT_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}
