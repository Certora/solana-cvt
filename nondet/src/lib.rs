#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod core;
mod scalars;
#[cfg(feature = "solana")]
mod solana;
#[cfg(feature = "std")]
mod aggregates;
#[cfg(feature = "soroban")]
mod soroban;

pub use crate::core::*;
#[cfg(feature = "solana")]
pub use solana::*;
#[cfg(feature = "std")]
pub use aggregates::*;
#[cfg(feature = "soroban")]
mod soroban;