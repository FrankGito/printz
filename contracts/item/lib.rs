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
        token_owner: Mapping<Id, AccountId>,
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
            todo!();
        }

        #[ink(message)]
        fn total_supply(&self) -> u128 {
            self.total_supply
        }

        #[ink(message)]
        fn balance_of(&self, owner: AccountId) -> u32 {
            self.owned_tokens_count.get(owner).unwrap_or(0)
        }

        #[ink(message)]
        fn allowance(&self, _owner: AccountId, _operator: AccountId, _id: Option<Id>) -> bool {
            todo!();
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, _data: Vec<u8>) -> Result<(), PSP34Error> {
            let owner = self.owner_of(id).ok_or(PSP34Error::TokenNotExists)?;

            if owner == to {
                return Ok(());
            }

            let current_count = self.owned_tokens_count.get(owner).unwrap_or_default();
            let new_balance = current_count - 1;
            self.owned_tokens_count.insert(owner, &new_balance);

            let next_owner = to;
            self.token_owner.insert(id, &next_owner);
            Ok(())
        }

        #[ink(message)]
        fn approve(
            &mut self,
            _operator: AccountId,
            _id: Option<Id>,
            _approved: bool,
        ) -> Result<(), PSP34Error> {
            todo!();
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.token_owner.get(id)
        }
    }
}
