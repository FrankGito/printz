#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use interfaces::psp34_error::PSP34Error;
pub use interfaces::psp34_metadata::Psp34Metadata;
pub use interfaces::psp34_mintable::Psp34Mintable;

#[ink::contract]
mod item {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    type Uri = String;
    #[ink(storage)]
    #[derive(Default)]
    pub struct Item {
        owned_tokens_count: Mapping<AccountId, u32>,
        total_supply: u128,
        uri: Uri,
    }

    impl Item {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }
        #[ink(message)]
        pub fn get_uri(&self) -> Uri {
            self.uri.clone()
        }
    }
}
