use crate::Nondet;

crate::nondet_impl! { (), (),  "No nondet value for  unit" }
crate::nondet_impl! { bool, certora::CERTORA_nondet_u64() > 0, "Nondet for bool"}
crate::nondet_impl! { u8, certora::CERTORA_nondet_u8(), "Nondet for u8" }
crate::nondet_impl! { i8, certora::CERTORA_nondet_i8(), "Nondet for i8" }
crate::nondet_impl! { u16, certora::CERTORA_nondet_u16(), "Nondet for u16" }
crate::nondet_impl! { i16, certora::CERTORA_nondet_i16(), "Nondet for i16" }
crate::nondet_impl! { u32, certora::CERTORA_nondet_u32(), "Nondet for u32" }
crate::nondet_impl! { i32, certora::CERTORA_nondet_i32(), "Nondet for i32" }
crate::nondet_impl! { u64, certora::CERTORA_nondet_u64(), "Nondet for u64" }
crate::nondet_impl! { i64, certora::CERTORA_nondet_i64(), "Nondet for i64" }
crate::nondet_impl! { usize, certora::CERTORA_nondet_usize(), "Nondet for usize" }