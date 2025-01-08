/// ! Mathematical Integers
/// 
/// This crate provides various representations of integers that behave like
/// mathematical integers that do not overflow. The functionality is exposed via
/// [MathInt] type.
/// 
/// Use feature `rt` to enable run-time under-approximation, for example, for testing.
pub mod mathint_u64;

pub use mathint_u64::MathIntU64 as MathInt;
