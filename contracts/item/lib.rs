#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use ::frank_test::frank_test;
pub use interfaces::psp34::Psp34;
pub use interfaces::psp34_error::PSP34Error;
pub use interfaces::psp34_mintable::Psp34Mintable;
pub use interfaces::Id;

#[ink::contract]
pub mod item {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use interfaces::psp34::Psp34;
    use interfaces::psp34_error::PSP34Error;
    use interfaces::psp34_mintable::Psp34Mintable;
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
            // Get caller
            let caller = self.env().caller();
            // Check if caller has NFT
            if self.token_owner.get(id).is_none() {
                return Err(PSP34Error::Custom(String::from("Caller doesn't have it")));
            }
            // Decrease owned_tokens_count
            let current_count_caller = self.owned_tokens_count.get(caller).unwrap_or(0);
            self.owned_tokens_count
                .insert(caller, &current_count_caller.saturating_sub(0));
            // set new token_owner
            self.token_owner.insert(id, &to);
            let current_count_to = self.owned_tokens_count.get(caller).unwrap_or(0);
            self.owned_tokens_count
                .insert(to, &current_count_to.saturating_add(1));
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
    impl Psp34Mintable for Item {
        #[ink(message)]
        fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            // Check if Id is already taken
            if self.token_owner.get(id).is_some() {
                return Err(PSP34Error::TokenExists);
            }

            // Get current caller
            let caller = self.env().caller();
            // Increase owned tokens_count
            let current_count = self.owned_tokens_count.get(caller).unwrap_or(0);
            let new_countr = current_count.saturating_add(1);
            self.owned_tokens_count.insert(caller, &new_countr);

            // Increase total_supply
            let current_supply = self.total_supply;
            let new_supply = current_supply.saturating_add(1);
            self.total_supply = new_supply;

            // add Owner
            self.token_owner.insert(id, &caller);
            Ok(())
        }
    }

    #[cfg(test)]
    mod frank_test {
        crate::frank_test!(Item, Item::new);
    }
    // #[cfg(test)]
    // pub mod tests {
    //     use super::*;
    //
    //     #[ink::test]
    //     fn alice_has_zero() {
    //         let item = Item::new();
    //         let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    //         let alice = accounts.alice;
    //         let count = item.owned_tokens_count.get(alice);
    //         assert_eq!(count, None);
    //     }
    //
    //     #[ink::test]
    //     fn balance_of_alice_is_zero() {
    //         let item = Item::new();
    //         let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    //         let alice = accounts.alice;
    //         let count = item.balance_of(alice);
    //         assert_eq!(count, 0u32);
    //     }
    //
    //     #[ink::test]
    //     fn total_supply_is_zero() {
    //         let item = Item::new();
    //         let total_supply = item.total_supply();
    //         assert_eq!(total_supply, 0)
    //     }
    //
    //     #[ink::test]
    //     fn mint_works() {
    //         let mut item = Item::new();
    //         let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    //         let alice = accounts.alice;
    //         ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
    //         let _ = item.mint(0);
    //         assert!(item.owner_of(0).unwrap() == alice);
    //     }
    //
    //     #[ink::test]
    //     fn transfer_works() {
    //         let mut item = Item::new();
    //         let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
    //         let alice = accounts.alice;
    //         let bob = accounts.bob;
    //         ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
    //         let _ = item.mint(0);
    //         let _ = item.transfer(bob, 1u128, Vec::new());
    //         let count = item.owned_tokens_count.get(bob);
    //         assert!(count.is_none())
    //     }
    // }
}
