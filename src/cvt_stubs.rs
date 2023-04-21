use {
    solana_program::{
        account_info::{AccountInfo},
        pubkey::Pubkey
    }
};

#[allow(non_snake_case)]
pub fn CVT_assume_impl(_c: bool){
    println!("you should never see this 1!");
}

#[allow(non_snake_case)]
pub fn CVT_assert_impl(_c: bool){
    println!("you should never see this 2!");
}

#[allow(non_snake_case)]
pub fn CVT_nondet_u8_impl() ->  u8 {
    println!("you should never see this 3!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_u32_impl() ->  u32 {
    println!("you should never see this 4!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_u64_impl() ->  u64 {
    println!("you should never see this 5!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_i8_impl() ->  i8 {
    println!("you should never see this 6!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_i32_impl() -> i32 {
    println!("you should never see this 7!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_i64_impl() ->  i64 {
    println!("you should never see this 8!");
    return 0;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_usize_impl() ->  usize {
    println!("you should never see this 9!");
    return 0;
}


extern "C" {
    fn CVT_nondet_pointer_usize() -> *mut usize;
}

static mut CVT_UNINTERPRETED_USIZE: *mut usize = std::ptr::null_mut();

#[allow(non_snake_case)]
pub fn CVT_uninterpreted_usize_impl() ->  usize {
    unsafe {
        if CVT_UNINTERPRETED_USIZE.is_null() {
            CVT_UNINTERPRETED_USIZE = CVT_nondet_pointer_usize()
        }
        *CVT_UNINTERPRETED_USIZE
    }
}

extern "C" {
    #[allow(improper_ctypes)]
    fn mk_account_info_unchecked() -> AccountInfo<'static>;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_account_info_impl() -> AccountInfo<'static> {
    unsafe {
        return mk_account_info_unchecked();
    }
}

extern "C" {
    #[allow(improper_ctypes)]
    fn mk_pubkey_unchecked() -> Pubkey;
}

#[allow(non_snake_case)]
pub fn CVT_nondet_pubkey_impl() -> Pubkey {
    unsafe {
        return mk_pubkey_unchecked();
    }
}