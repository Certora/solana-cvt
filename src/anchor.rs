use { 
    std::collections::BTreeMap,
    solana_program::account_info::AccountInfo,
    anchor_lang::prelude::{
    //    Account, 
    //    Program, 
        Signer,
    },
    crate::{
        cvt::CVT_nondet_btree_map,
        nondet::{
            Nondet,
            nondet_impl,
            nondet_with,
        }
    },
};

nondet_impl!{ BTreeMap<String,u8>, CVT_nondet_btree_map(), "Nondet for BtreeMap" }

impl Nondet for Signer<'static> {
    #[inline]
    fn nondet() -> Self {
        let acc_info = nondet_with::<AccountInfo<'static>,_>(|x| x.is_signer);
        Self::try_from(&acc_info).unwrap()
    }
}