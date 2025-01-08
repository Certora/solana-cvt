#![no_std]
use soroban_sdk::{Address};

mod callable;

pub use callable::Call;

extern "C" {
    fn CERTORA_SOROBAN_is_auth(address: u64) -> u64;

    // TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
    pub fn CERTORA_calltrace_print_c_u64 (s: &str, v: u64);
    pub fn CERTORA_calltrace_print_c_i64 (s: &str, v: i64);
    pub fn CERTORA_calltrace_print_c_u32 (s: &str, v: u32);
    pub fn CERTORA_calltrace_print_c_i32 (s: &str, v: i32);
}

pub fn is_auth(address: Address) -> bool {
    unsafe { CERTORA_SOROBAN_is_auth(address.to_val().get_payload()) != 0 }
}

// TODO: Would like to use the calltrace crate solana uses but dependency issues prevented it. 
#[macro_export]
macro_rules! certora_print_u64 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_u64($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_i64 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_i64($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_u32 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_u32($vnm, $x)}};
}

#[macro_export]
macro_rules! certora_print_i32 {
    ($vnm: expr, $x: expr) => { unsafe {CERTORA_calltrace_print_c_i32($vnm, $x)}};
}


/// This will not replace any recursive calls to the summarized function
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
