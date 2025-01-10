
#![no_std]
use stubs::certora_stubs;

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_assume(c: bool){
    certora_stubs::CERTORA_assume_impl(c)
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_assert(c: bool){
    certora_stubs::CERTORA_assert_impl(c)
}

#[inline(always)]
#[allow(non_snake_case)]
pub fn CERTORA_satisfy(c: bool){
    certora_stubs::CERTORA_satisfy_impl(c)
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u8() ->  u8 {
    certora_stubs::CERTORA_nondet_u8_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u16() ->  u16 {
    certora_stubs::CERTORA_nondet_u16_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u32() ->  u32 {
    certora_stubs::CERTORA_nondet_u32_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_u64() ->  u64 {
    certora_stubs::CERTORA_nondet_u64_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_usize() ->  usize { certora_stubs::CERTORA_nondet_usize_impl() }

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i8() ->  i8 {
    certora_stubs::CERTORA_nondet_i8_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i16() ->  i16 {
    certora_stubs::CERTORA_nondet_i16_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i32() -> i32 {
    certora_stubs::CERTORA_nondet_i32_impl()
}

#[inline(never)]
#[allow(non_snake_case)]
pub fn CERTORA_nondet_i64() ->  i64 {
    certora_stubs::CERTORA_nondet_i64_impl()
}

extern "C" {
    // Rust: we cannot use type parameters on foreign items
    // fn CERTORA_nondet_pointer_c<T: Nondet>() -> *mut T;
    pub fn CERTORA_nondet_pointer_usize() -> *mut usize;
}

#[macro_export]
/// Returns an arbitrary usize but always the same
macro_rules! certora_deterministic_usize {
    ($fname:ident, $gname: ident) => {
        static mut $gname: *mut usize = std::ptr::null_mut();
        #[allow(non_snake_case)]
	/// Returns an arbitrary usize but always the same
        pub fn $fname() ->  usize {
            unsafe {
                if $gname.is_null() {
                    $gname = certora::CERTORA_nondet_pointer_usize()
                }
                *$gname
             }
        }
    };
}

#[inline(never)]
#[allow(non_snake_case)]
/// Return an array of 32 bytes initialized non-deterministically
pub fn CERTORA_nondet_array_of_32_bytes() -> [u8; 32] {
    certora_stubs::CERTORA_nondet_array_of_32_bytes_impl()
}

/// We redefine these macros to avoid including error conversion/formatting code
#[macro_export]
macro_rules! require {
    ($invariant:expr, $error:tt $(,)?) => {
        certora::CERTORA_assume($invariant);
    };
    ($invariant:expr, $error:expr $(,)?) => {
        certora::CERTORA_assume($invariant);
    };
}

#[macro_export]
macro_rules! require_gte {
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        certora::CERTORA_assume($value1 >= $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        certora::CERTORA_assume($value1 >= $value2);
    };
}

#[macro_export]
macro_rules! require_eq {
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        certora::CERTORA_assume($value1 == $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        certora::CERTORA_assume($value1 == $value2);
    };
}

#[macro_export]
macro_rules! require_neq {
    ($value1: expr, $value2: expr, $error_code: expr $(,)?) => {
        certora::CERTORA_assume($value1 != $value2);
    };
    ($value1: expr, $value2: expr $(,)?) => {
        certora::CERTORA_assume($value1 != $value2);
    };
}

#[macro_export]
macro_rules! assert {
        ($cond:expr) => {{ certora::CERTORA_assert($cond)}};
}

#[macro_export]
macro_rules! satisfy {
        ($cond:expr) => {{ certora::CERTORA_satisfy($cond)}};
}

#[macro_export]
macro_rules! apply_summary {
    (@mk_orig_module $id:ident, [$($prototype:tt)*], $( -> $ret:ty )?, $body:block) => {
        #[cfg(feature="certora")]
        pub(crate) mod $id {
            use super::*;
            #[allow(dead_code)]
            #[allow(unused_variables)]
            pub(crate) fn $id($($prototype)*) $( -> $ret )? $body
        }
    };
    (@mk_orig $( #[$meta:meta] )*, $id:ident, [$($prototype:tt)*], $( -> $ret:ty )?, $body:block) => {
        $( #[$meta] )*
        $vis fn $id($($prototype)*) $( -> $ret )? $body
    };
    ($new:path,
        $( #[$meta:meta]  )*
        $vis:vis fn $id:ident ($($arg:ident : $arg_ty:ty),* $(,)?) $( -> $ret:ty )?
        $body:block
    ) => {
        #[cfg(feature="certora")]
        $( #[$meta] )*
        pub(crate) fn $id($($arg : $arg_ty),*) $( -> $ret )? {
            $new($($arg),*)
        }

        $crate::apply_summary!(
            @mk_orig_module $id, [$($arg : $arg_ty),*], $( -> $ret  )?, $body
        );

        #[cfg(not(feature="certora"))]
        $crate::apply_summary!(@mk_orig $( #[$meta] )*, $id, [$($arg : $arg_ty),*], $( -> $ret  )?, $body);
    };

    ($spec:ident, $old:ident,
        $( #[$meta:meta]  )*
        $vis:vis fn $id:ident (&mut $self:ident $( , )? $($arg:ident : $arg_ty:ty),* $(,)?) $( -> $ret:ty )?
        $body:block
    ) => {
        #[cfg(feature="certora")]
        $( #[$meta] )*
        pub(crate) fn $id(&mut $self, $($arg : $arg_ty),*) $( -> $ret )? {
            $self.$spec($($arg),*)
        }

        pub(crate) fn $old(&mut $self, $($arg : $arg_ty),*) $( -> $ret )? $body

        #[cfg(not(feature="certora"))]
        $crate::apply_summary!(@mk_orig $( #[$meta] )*, $id, [&mut $self, $($arg : $arg_ty),*], $( -> $ret  )?, $body);
    };

    ($spec:ident, $old:ident,
        $( #[$meta:meta]  )*
        $vis:vis fn $id:ident (&$self:ident $( , )? $($arg:ident : $arg_ty:ty),* $(,)?) $( -> $ret:ty )?
        $body:block
    ) => {
        #[cfg(feature="certora")]
        $( #[$meta] )*
        pub(crate) fn $id(&$self, $($arg : $arg_ty),*) $( -> $ret )? {
            $self.$spec($($arg),*)
        }

        pub(crate) fn $old(&$self, $($arg : $arg_ty),*) $( -> $ret )? $body

        #[cfg(not(feature="certora"))]
        $crate::apply_summary!(@mk_orig $( #[$meta] )*, $id, [&$self, $($arg : $arg_ty),*], $( -> $ret  )?, $body);
    };

    ($spec:ident, $old:ident,
        $( #[$meta:meta]  )*
        $vis:vis fn $id:ident ($self:ident $( , )? $($arg:ident : $arg_ty:ty),* $(,)?) $( -> $ret:ty )?
        $body:block
    ) => {
        #[cfg(feature="certora")]
        $( #[$meta] )*
        pub(crate) fn $id($self, $($arg : $arg_ty),*) $( -> $ret )? {
            $self.$spec($($arg),*)
        }

        pub(crate) fn $old($self, $($arg : $arg_ty),*) $( -> $ret )? $body

        #[cfg(not(feature="certora"))]
        $crate::apply_summary!(@mk_orig $( #[$meta] )*, $id, [$self, $($arg : $arg_ty),*], $( -> $ret  )?, $body);
    };
}
