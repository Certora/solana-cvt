#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod aggregates;
mod core;
mod scalars;
#[cfg(feature = "solana")]
mod solana;

pub use crate::core::*;
#[cfg(feature = "std")]
pub use aggregates::*;
#[cfg(feature = "solana")]
pub use solana::*;

#[cfg(feature = "derive")]
pub use derive_nondet::*;
