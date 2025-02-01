use solana_program::account_info::AccountInfo;
use cvlr_log::clog;

#[macro_export]
macro_rules! clog_acc_info {
    ($acc: expr) => {
        $crate::cvlr_clog_account_info(stringify!($acc), $acc);
    };
}


#[inline(always)]
pub fn cvlr_clog_account_info(tag: &str, acc_info: &AccountInfo) {
    clog!(tag);
    clog!(acc_info.lamports());
    clog!(acc_info.data_len());
    clog!(acc_info.is_signer);
    clog!(acc_info.is_writable);
    
}