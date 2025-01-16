#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
mod aggregates;

#[cfg(feature = "std")]
pub mod havoc;

mod core;
mod scalars;

#[cfg(feature = "solana")]
mod solana;

#[cfg(feature = "spl_token")]
pub mod spl_token;

pub use crate::core::*;
pub use scalars::*;

#[cfg(feature = "std")]
pub use aggregates::*;

#[cfg(feature = "solana")]
pub use solana::*;

#[cfg(feature = "derive")]
pub use derive_nondet::*;
