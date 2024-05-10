#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use interfaces::psp34::Psp34;
pub use interfaces::psp34_error::PSP34Error;
pub use interfaces::Id;

#[ink::contract]
mod item {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use interfaces::psp34::Psp34;
    use interfaces::psp34_error::PSP34Error;
    use interfaces::Id;

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

    impl Psp34 for Item {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            5u128
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            5u128
        }

        #[ink(message)]
        fn balance_of(&self, _owner: AccountId) -> u32 {
            5u32
        }

        #[ink(message)]
        fn allowance(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>) -> bool {
            true
        }

        #[ink(message)]
        fn transfer(&mut self, _to: AccountId, _id: Id, _data: Vec<u8>) -> Result<(), PSP34Error> {
            Ok(())
        }

        #[ink(message)]
        fn approve(
            &mut self,
            _operator: AccountId,
            _id: Option<Id>,
            _approved: bool,
        ) -> Result<(), PSP34Error> {
            Ok(())
        }

        #[ink(message)]
        fn owner_of(&self, _id: Id) -> Option<AccountId> {
            Some(AccountId::from([0xFF; 32]))
        }
    }
}
