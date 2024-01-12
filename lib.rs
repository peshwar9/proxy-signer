#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod proxy_signer_whitelist {
    use ink::storage::{Mapping};
    use ink::{
        prelude::{vec::Vec, string::String},
    };

    #[ink(storage)]
    pub struct ProxySignerWhitelist {
        whitelist: Mapping<u32, WhitelistEntry>,
        admin_accounts: Mapping<AccountId, bool>,
    }

    pub type NftId = u32;

    #[derive(Debug, Clone, scale::Encode, scale::Decode, PartialEq, Eq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]    
    pub struct WhitelistEntry {
        whitelisted_address: AccountId,
        is_restricted: bool,
        operations: Vec<String>,
    }

    impl ProxySignerWhitelist {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                whitelist: Mapping::new(),
                admin_accounts: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn add_proxy_signer(&mut self, account: AccountId) {
            self.admin_accounts.insert(account, &true);
        }

        #[ink(message)]
        pub fn remove_proxy_signer(&mut self, account: AccountId) {
            self.admin_accounts.take(&account);
        }

        #[ink(message)]
        pub fn add_to_whitelist(&mut self, nft_id: NftId, entry: WhitelistEntry) {
            if self.admin_accounts.get(&self.env().caller()).is_some() {
                self.whitelist.insert(nft_id, &entry);
            }
        }

        #[ink(message)]
        pub fn remove_from_whitelist(&mut self, nft_id: NftId) {
            if self.admin_accounts.get(&self.env().caller()).is_some() {
                self.whitelist.take(&nft_id);
            }
        }

        #[ink(message)]
        pub fn get_whitelist_entry(&self, nft_id: NftId) -> Option<(bool, Vec<String>)> {
            self.whitelist.get(&nft_id).map(|entry| {
                (entry.is_restricted, entry.operations.clone())
            })
        }
      /*  #[ink(message)]
        pub fn get_proxy_signers(&self) -> Option<(AccountId)> {
            self.admin_accounts.get().map(|account| {
                (account)
            })
        }    */    
    }
}
