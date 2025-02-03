use crate::nondet::cvlr_nondet_account_info;
use solana_program::account_info::AccountInfo;

/**

The function `fun_acc_infos_with_mem_layout` returns 16 AccountInfo
initialized non-deterministically.

While the contents of the accounts are unconstrained, this function
assigns fixed addresses to any field that contains a pointer or
reference: `key`, `lamports`, `data`, and `owner`.

The purpose of this is to eliminate spurious counterexamples while
making easier the debugging of counterexamples.  This assignment of
fixed addresses should be sound since no Solana contract should branch
on whether, for instance, a public key starts at some particular
address in the SVM (Solana Virtual Machine).

```
 pub struct AccountInfo<'a> {
     /// Public key of the account
     pub key: &'a Pubkey,
     /// The lamports in the account.  Modifiable by programs.
     pub lamports: Rc<RefCell<&'a mut u64>>,
     /// The data held in this account.  Modifiable by programs.
     pub data: Rc<RefCell<&'a mut [u8]>>,
     /// Program that owns this account
     pub owner: &'a Pubkey,
     /// The epoch at which this account will next owe rent
     pub rent_epoch: Epoch,
     /// Was the transaction signed by this account's public key?
     pub is_signer: bool,
     /// Is the account writable?
     pub is_writable: bool,
     /// This account's data contains a loaded program (and is now read-only)
     pub executable: bool
 }
```
 **/

/// Memory layout of AccountInfo field `data` as `Rc<RefCell<&[u8]>>`
macro_rules! mem_layout_rc_data {
    ($acc_info: expr, $start_addr: expr, $num_acc: expr) => {{
        // We need the start of the Rc. The method as_ptr() returns the address of &[u8]
        let data_rc = unsafe { $acc_info.data.as_ptr().offset(-24) } as *const _;
        cvlr_asserts::cvlr_assume!(data_rc == ($start_addr + (512 * $num_acc)) as *const _);
    }};
}

/// Memory layout of AccountInfo field `data` as `[u8]`
macro_rules! mem_layout_data {
    ($acc_info_prev: expr, $acc_info: expr, $data_sz: expr) => {{
        let prev_data_ptr = $acc_info_prev.data.borrow().as_ptr();
        let data_ptr = $acc_info.data.borrow().as_ptr();
        cvlr_asserts::cvlr_assume!((prev_data_ptr as usize + $data_sz) as *const u8 == data_ptr);
    }};
}

/// Memory layout of AccountInfo field `lamports` as `Rc<RefCell<&u64>>`
macro_rules! mem_layout_rc_lamport {
    ($acc_info: expr, $start_addr: expr, $num_acc: expr) => {{
        // We need the start of the Rc. The method as_ptr() returns the address of T
        let lamports_rc = unsafe { $acc_info.lamports.as_ptr().offset(-24) } as *const _;
        cvlr_asserts::cvlr_assume!(lamports_rc == ($start_addr + (64 * $num_acc)) as *const _);
    }};
}

/// Memory layout of AccountInfo field `lamports` as `&u64`
macro_rules! mem_layout_lamport {
    ($acc_info_prev: expr, $acc_info: expr, $data_sz: expr) => {{
        let prev_lamports_ptr = *$acc_info_prev.lamports.borrow() as *const u64;
        let lamports_ptr = *$acc_info.lamports.borrow() as *const u64;
        cvlr_asserts::cvlr_assume!(
            (prev_lamports_ptr as usize + $data_sz) as *const u64 == lamports_ptr
        );
    }};
}

/// Memory layout of AccountInfo field `key` as `&Pubkey`
macro_rules! mem_layout_key {
    ($acc_info: expr, $start_addr: expr, $num_acc: expr) => {{
        let key_ptr = &*$acc_info.key as *const _;
        cvlr_asserts::cvlr_assume!(key_ptr as usize == $start_addr + (64 * $num_acc));
    }};
}

/// Memory layout of AccountInfo field `owner` as `&Pubkey`
macro_rules! mem_layout_owner {
    ($acc_info: expr, $start_addr: expr, $num_acc: expr) => {{
        let owner_ptr = &*$acc_info.owner as *const _;
        cvlr_asserts::cvlr_assume!(owner_ptr as usize == $start_addr + (64 * $num_acc));
    }};
}

pub fn fun_acc_infos_with_mem_layout() -> [AccountInfo<'static>; 16] {
    let acc1 = cvlr_nondet_account_info();
    let acc2 = cvlr_nondet_account_info();
    let acc3 = cvlr_nondet_account_info();
    let acc4 = cvlr_nondet_account_info();
    let acc5 = cvlr_nondet_account_info();
    let acc6 = cvlr_nondet_account_info();
    let acc7 = cvlr_nondet_account_info();
    let acc8 = cvlr_nondet_account_info();
    let acc9 = cvlr_nondet_account_info();
    let acc10 = cvlr_nondet_account_info();
    let acc11 = cvlr_nondet_account_info();
    let acc12 = cvlr_nondet_account_info();
    let acc13 = cvlr_nondet_account_info();
    let acc14 = cvlr_nondet_account_info();
    let acc15 = cvlr_nondet_account_info();
    let acc16 = cvlr_nondet_account_info();

    /*
     *   When the Solana program entrypoint is called the contents of
     *   the AccountInfo are lay out in the "context" (aka "input")
     *   memory region like this:
     *
     *           | AccountInfo 1 | AccountInfo 2 | .... | AccountInfo N |
     *
     *   For convenience, we arrange them differently. First all the data fields, then all the lamport fields, and so on.
     *
     *
     *               16                               16                                   16                 16                 16              16
     *   data &[u8] ... data &[u8] | lamports Rc  ... lamports Rc | lamports &u64 ... lamports &u64 | data Rc ... data Rc  | key ... key   | owner ... owner
     *    ^                           ^                              ^                                 ^                      ^               ^
     *   0x400_000_008             | 0x40A_000_080                | 0x40A_000_480                   | 0x40A_000_500        | 0x40A_002_500 | 0x40A_002_900
     *
     *   <----  16*0xA00_008 -----><----------  16*64=0x400 ------><------------ 16*8=0x80----------><----16*512=0x2000----><-----0x400---><-----0x400----->
     *
     **/
    {
        // layout of data &[u8]
        // The actual address is 0x400_000_000 and it's the start of the context memory region in SVM
        let start_addr: u64 = 0x400_000_008;
        // each account has size of 10MB: 10485760  (0xA00_000). We add 8 just to be conservative.
        let data_sz: usize = 10485760 + 8;

        let acc1_data_ptr = acc1.data.borrow().as_ptr();
        cvlr_asserts::cvlr_assume!(acc1_data_ptr == start_addr as *const u8);
        mem_layout_data!(acc1, acc2, data_sz);
        mem_layout_data!(acc2, acc3, data_sz);
        mem_layout_data!(acc3, acc4, data_sz);
        mem_layout_data!(acc4, acc5, data_sz);
        mem_layout_data!(acc5, acc6, data_sz);
        mem_layout_data!(acc6, acc7, data_sz);
        mem_layout_data!(acc7, acc8, data_sz);
        mem_layout_data!(acc8, acc9, data_sz);
        mem_layout_data!(acc9, acc10, data_sz);
        mem_layout_data!(acc10, acc11, data_sz);
        mem_layout_data!(acc11, acc12, data_sz);
        mem_layout_data!(acc12, acc13, data_sz);
        mem_layout_data!(acc13, acc14, data_sz);
        mem_layout_data!(acc14, acc15, data_sz);
        mem_layout_data!(acc15, acc16, data_sz);
    }
    {
        // layout of lamports Rc<RefCell<T>>
        let start_addr: usize = 0x40A_000_080;
        mem_layout_rc_lamport!(acc1, start_addr, 0);
        mem_layout_rc_lamport!(acc2, start_addr, 1);
        mem_layout_rc_lamport!(acc3, start_addr, 2);
        mem_layout_rc_lamport!(acc4, start_addr, 3);
        mem_layout_rc_lamport!(acc5, start_addr, 4);
        mem_layout_rc_lamport!(acc6, start_addr, 5);
        mem_layout_rc_lamport!(acc7, start_addr, 6);
        mem_layout_rc_lamport!(acc8, start_addr, 7);
        mem_layout_rc_lamport!(acc9, start_addr, 8);
        mem_layout_rc_lamport!(acc10, start_addr, 9);
        mem_layout_rc_lamport!(acc11, start_addr, 10);
        mem_layout_rc_lamport!(acc12, start_addr, 11);
        mem_layout_rc_lamport!(acc13, start_addr, 12);
        mem_layout_rc_lamport!(acc14, start_addr, 13);
        mem_layout_rc_lamport!(acc15, start_addr, 14);
        mem_layout_rc_lamport!(acc16, start_addr, 15);
    }
    {
        // layout of lamports
        let start_addr: usize = 0x40A_000_480;
        let acc1_lamports_ptr = *acc1.lamports.borrow() as *const u64;
        cvlr_asserts::cvlr_assume!(acc1_lamports_ptr == start_addr as *const u64);

        mem_layout_lamport!(acc1, acc2, 8);
        mem_layout_lamport!(acc2, acc3, 8);
        mem_layout_lamport!(acc3, acc4, 8);
        mem_layout_lamport!(acc4, acc5, 8);
        mem_layout_lamport!(acc5, acc6, 8);
        mem_layout_lamport!(acc6, acc7, 8);
        mem_layout_lamport!(acc7, acc8, 8);
        mem_layout_lamport!(acc8, acc9, 8);
        mem_layout_lamport!(acc9, acc10, 8);
        mem_layout_lamport!(acc10, acc11, 8);
        mem_layout_lamport!(acc11, acc12, 8);
        mem_layout_lamport!(acc12, acc13, 8);
        mem_layout_lamport!(acc13, acc14, 8);
        mem_layout_lamport!(acc14, acc15, 8);
        mem_layout_lamport!(acc15, acc16, 8);
    }
    {
        // layout of data Rc<RefCell<...>>
        let start_addr: usize = 0x40A_000_500;
        mem_layout_rc_data!(acc1, start_addr, 0);
        mem_layout_rc_data!(acc2, start_addr, 1);
        mem_layout_rc_data!(acc3, start_addr, 2);
        mem_layout_rc_data!(acc4, start_addr, 3);
        mem_layout_rc_data!(acc5, start_addr, 4);
        mem_layout_rc_data!(acc6, start_addr, 5);
        mem_layout_rc_data!(acc7, start_addr, 6);
        mem_layout_rc_data!(acc8, start_addr, 7);
        mem_layout_rc_data!(acc9, start_addr, 8);
        mem_layout_rc_data!(acc10, start_addr, 9);
        mem_layout_rc_data!(acc11, start_addr, 10);
        mem_layout_rc_data!(acc12, start_addr, 11);
        mem_layout_rc_data!(acc13, start_addr, 12);
        mem_layout_rc_data!(acc14, start_addr, 13);
        mem_layout_rc_data!(acc15, start_addr, 14);
        mem_layout_rc_data!(acc16, start_addr, 15);
    }
    {
        // layout of key
        let start_addr: usize = 0x40A_002_500;
        mem_layout_key!(acc1, start_addr, 0);
        mem_layout_key!(acc2, start_addr, 1);
        mem_layout_key!(acc3, start_addr, 2);
        mem_layout_key!(acc4, start_addr, 3);
        mem_layout_key!(acc5, start_addr, 4);
        mem_layout_key!(acc6, start_addr, 5);
        mem_layout_key!(acc7, start_addr, 6);
        mem_layout_key!(acc8, start_addr, 7);
        mem_layout_key!(acc9, start_addr, 8);
        mem_layout_key!(acc10, start_addr, 9);
        mem_layout_key!(acc11, start_addr, 10);
        mem_layout_key!(acc12, start_addr, 11);
        mem_layout_key!(acc13, start_addr, 12);
        mem_layout_key!(acc14, start_addr, 13);
        mem_layout_key!(acc15, start_addr, 14);
        mem_layout_key!(acc16, start_addr, 15);
    }
    {
        // layout of owner
        let start_addr: usize = 0x40A_002_900;
        mem_layout_owner!(acc1, start_addr, 0);
        mem_layout_owner!(acc2, start_addr, 1);
        mem_layout_owner!(acc3, start_addr, 2);
        mem_layout_owner!(acc4, start_addr, 3);
        mem_layout_owner!(acc5, start_addr, 4);
        mem_layout_owner!(acc6, start_addr, 5);
        mem_layout_owner!(acc7, start_addr, 6);
        mem_layout_owner!(acc8, start_addr, 7);
        mem_layout_owner!(acc9, start_addr, 8);
        mem_layout_owner!(acc10, start_addr, 9);
        mem_layout_owner!(acc11, start_addr, 10);
        mem_layout_owner!(acc12, start_addr, 11);
        mem_layout_owner!(acc13, start_addr, 12);
        mem_layout_owner!(acc14, start_addr, 13);
        mem_layout_owner!(acc15, start_addr, 14);
        mem_layout_owner!(acc16, start_addr, 15);
    }

    return [
        acc1, acc2, acc3, acc4, acc5, acc6, acc7, acc8, acc9, acc10, acc11, acc12, acc13, acc14,
        acc15, acc16,
    ];
}

// #[macro_export]
// macro_rules! acc_infos_with_mem_layout {
//     () => {
//         $crate::fun_acc_infos_with_mem_layout()
//     };
// }

#[macro_export]
macro_rules! acc_infos_with_mem_layout {
    () => {
        $crate::cvlr_deserialize_nondet_accounts()
    };
}




pub fn cvlr_new_account_info() -> AccountInfo<'static> {
    unsafe { cvlr_new_account_info_unchecked() }
}

mod rt_decls {
    extern "C" {
        pub fn CVT_nondet_solana_account_space(size: usize) -> *mut u8;
    }
}

#[cfg(feature = "rt")]
mod rt_impls {
    use solana_program::entrypoint::BPF_ALIGN_OF_U128;
    use std::alloc::{alloc_zeroed, Layout};

    #[no_mangle]
    extern "C" fn CVT_nondet_solana_account_space(size: usize) -> *mut u8 {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, BPF_ALIGN_OF_U128);
            let input = alloc_zeroed(layout);
            input
        }
    }
}

#[allow(unused_assignments)]
unsafe fn cvlr_new_account_info_unchecked() -> AccountInfo<'static> {
    use solana_program::{
        entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE},
        pubkey::Pubkey,
    };
    use std::{alloc::Layout, cell::RefCell, mem::size_of, rc::Rc};

    const MB: usize = 1024 * 1024;
    const MAX_ORIG_DATA_LEN: usize = 8 * MB;
    const SIZE: usize =
        4 + 4 + 32 + 32 + 8 + 8 + MAX_ORIG_DATA_LEN + MAX_PERMITTED_DATA_INCREASE + 8;

    let layout = Layout::from_size_align_unchecked(SIZE, BPF_ALIGN_OF_U128);
    let input: *mut u8 = rt_decls::CVT_nondet_solana_account_space(layout.size());

    let mut offset: usize = 0;

    offset += size_of::<u8>();

    let is_signer = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    let is_writable = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    let executable = *(input.add(offset) as *const u8) != 0;
    offset += size_of::<u8>();

    let original_data_len_offset = offset;
    offset += size_of::<u32>();

    let key: &Pubkey = &*(input.add(offset) as *const Pubkey);
    offset += size_of::<Pubkey>();

    let owner: &Pubkey = &*(input.add(offset) as *const Pubkey);
    offset += size_of::<Pubkey>();

    let lamports = Rc::new(RefCell::new(&mut *(input.add(offset) as *mut u64)));
    offset += size_of::<u64>();

    let data_len = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    *(input.add(original_data_len_offset) as *mut u32) = data_len as u32;

    // -- limit size of data to what is allocated
    cvlr_asserts::cvlr_assume!(data_len <= MAX_ORIG_DATA_LEN);

    let data = Rc::new(RefCell::new(
        std::slice::from_raw_parts_mut(input.add(offset), data_len)
    ));

    offset += data_len + MAX_PERMITTED_DATA_INCREASE;
    offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128);

    // let rent_epoch = *(input.add(offset) as *const u64);
    let rent_epoch = cvlr_nondet::nondet::<u64>();
    offset += size_of::<u64>();

    AccountInfo {
        key,
        is_signer,
        is_writable,
        lamports,
        data,
        owner,
        executable,
        rent_epoch,
    }
}

pub fn cvlr_deserialize_nondet_accounts() -> [AccountInfo<'static>; 16]  {
    [
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
        cvlr_new_account_info(),
    ]
}

