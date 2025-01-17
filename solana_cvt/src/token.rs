/// Summaries for Token and Token-2022
///
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError
};

use cvlr_mathint::NativeInt as MathInt;
use cvlr_asserts::cvlr_assume;

/// Unpack only amount from account [info] base
pub fn spl_token_account_get_amount(info: &AccountInfo) -> u64 {
    let data = info.data.borrow_mut();
    let base = array_ref![*data, 0, 165];
    let (_mint, _owner, amount, _delegate, _state, _is_native, _delegated_amount, _close_authority) =
        array_refs![base, 32, 32, 8, 36, 1, 12, 8, 36];
    let val = u64::from_le_bytes(*amount);
    cvlr_assume!(MathInt::from(val) <= MathInt::u64_max());
    val
}

/// Pack only [amount] from account [info] base
pub fn spl_token_account_set_amount(amount: u64, info: &AccountInfo) {
    let mut data = info.data.borrow_mut();
    let dst = array_mut_ref![*data, 0, 165];
    let (
        _mint_dst,
        _owner_dst,
        amount_dst,
        _delegate_dst,
        _state_dst,
        _is_native_dst,
        _delegated_amount_dst,
        _close_authority_dst,
    ) = mut_array_refs![dst, 32, 32, 8, 36, 1, 12, 8, 36];
    *amount_dst = amount.to_le_bytes();
}

/// Unpack only supply from [mint] base
pub fn spl_mint_get_supply(mint: &AccountInfo) -> u64 {
    let data = mint.data.borrow_mut();
    let src = array_ref![*data, 0, 82];
    let (_mint_authority, supply, _decimals, _is_initialized, _freeze_authority) =
        array_refs![src, 36, 8, 1, 1, 36];
    let supply =  u64::from_le_bytes(*supply);
    cvlr_assume!(MathInt::from(supply) <= MathInt::u64_max());
    supply
}

/// Unpack only decimals from [mint] base
pub fn spl_mint_get_decimals(mint: &AccountInfo) -> u8 {
    let data = mint.data.borrow_mut();
    let src = array_ref![*data, 0, 82];
    let (_mint_authority, _supply, decimals, _is_initialized, _freeze_authority) =
        array_refs![src, 36, 8, 1, 1, 36];
    let decimals = decimals[0];
    cvlr_assume!(MathInt::from(decimals as u64) <= MathInt::from(255u64));
    decimals
}


/// Pack only [supply] from [mint] base
pub fn spl_mint_set_supply(supply: u64, mint: &AccountInfo) {
    let mut data = mint.data.borrow_mut();
    let dst = array_mut_ref![*data, 0, 82];
    let (
        _mint_authority_dst,
        supply_dst,
        _decimals_dst,
        _is_initialized_dst,
        _freeze_authority_dst,
    ) = mut_array_refs![dst, 36, 8, 1, 1, 36];
    *supply_dst = supply.to_le_bytes();
}

/// Transfer [amount] from [src_info] to [dst_info] without any check
fn spl_transfer<'a>(
    src_info: &AccountInfo<'a>,
    dst_info: &AccountInfo<'a>,
    _authority_info: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    // non-op if self-transfer
    if src_info.key != dst_info.key {
        let mut src_amount = spl_token_account_get_amount(src_info);
        let mut dst_amount = spl_token_account_get_amount(dst_info);

        // source has enough founds
        cvlr_asserts::cvlr_assume!(src_amount >= amount);

        src_amount = src_amount.checked_sub(amount).unwrap();
        dst_amount = dst_amount.checked_add(amount).unwrap();

        spl_token_account_set_amount(src_amount, src_info);
        spl_token_account_set_amount(dst_amount, dst_info);
    }

    Ok(())
}

/// Summary for SPL Token transfer instruction
pub fn spl_token_transfer<'a>(
    src_info: &AccountInfo<'a>,
    dst_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    spl_transfer(src_info, dst_info, authority_info, amount)
}

/// Summary for SPL Token 2022 transfer instruction
/// This summary ignores extensions from SPL Token 2022 (e.g., fees)
pub fn spl_token_2022_transfer<'a>(
    src_info: &AccountInfo<'a>,
    dst_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    spl_transfer(src_info, dst_info, authority_info, amount)
}


/// Summary for SPL Token MintTo instruction
pub fn spl_mint_to<'a> (
    mint_info: &AccountInfo<'a>,
    dst_info: &AccountInfo<'a>,
    _authority: &AccountInfo<'a>,
    amount: u64
) -> ProgramResult {

    let mut mint_supply = spl_mint_get_supply(mint_info);
    let mut dst_amount = spl_token_account_get_amount(dst_info);

    mint_supply = mint_supply.checked_add(amount).unwrap();
    dst_amount = dst_amount.checked_add(amount).unwrap();

    spl_mint_set_supply(mint_supply, mint_info);
    spl_token_account_set_amount(dst_amount, dst_info);

    Ok(())
}

/// Summary for SPL Token Burn instruction
pub fn spl_burn<'a> (
    mint_info: &AccountInfo<'a>,
    src_info: &AccountInfo<'a>,
    _authority: &AccountInfo<'a>,
    amount: u64
) -> ProgramResult {

    let mut mint_supply = spl_mint_get_supply(mint_info);
    let mut src_amount = spl_token_account_get_amount(src_info);

    // -- enough funds to burn
    cvlr_asserts::cvlr_assume!(amount >= src_amount);

    mint_supply = mint_supply.checked_sub(amount).unwrap();
    src_amount = src_amount.checked_sub(amount).unwrap();

    spl_mint_set_supply(mint_supply, mint_info);
    spl_token_account_set_amount(src_amount, src_info);

    Ok(())
}

/// Summary for SPL Token close_account instruction
pub fn spl_close_account<'a> (
    src_info: &AccountInfo<'a>,
    dst_info: &AccountInfo<'a>,
    _authority_info: &AccountInfo<'a>
) -> ProgramResult {
    if src_info.key != dst_info.key {
        return Err(ProgramError::InvalidAccountData)
    }

    let src_amount = spl_token_account_get_amount(src_info);

    if src_amount != 0 {
        // The error code in SPL Token 2022 is TokenError::NonNativeHasBalance
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}    
