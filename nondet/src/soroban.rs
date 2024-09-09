
use soroban_sdk::{Address, Env, TryFromVal, Val};

use nondet::{*};

impl Nondet for Val {
    fn nondet() -> Self {
        let v: u64 = u64::nondet();
        Val::try_from_val(&Env::default(), &(&(v << 8) | (77 as u64))).unwrap()
    }
    
}

impl Nondet for Address {
    fn nondet() -> Self {
        Address::try_from_val(&Env::default(), &Val::nondet()).unwrap()
    }
}


