#![cfg_attr(not(feature = "std"), no_std, no_main)]

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
        collection_id: Id,
        owned_tokens_count: Mapping<AccountId, u32>,
        total_supply: u128,
        token_owner: Mapping<Id, AccountId>,
        uris: Mapping<Id, Uri>,
        approvals: Mapping<(AccountId, AccountId, Option<Id>), ()>,
    }

    impl Item {
        #[ink(constructor)]
        pub fn new() -> Self {
            // Default::default()
            Self {
                collection_id: 0,
                owned_tokens_count: Mapping::new(),
                total_supply: 0,
                token_owner: Mapping::new(),
                uris: Mapping::new(),
                approvals: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn get_uri(&self, id: Id) -> Result<Uri, PSP34Error> {
            if self.uris.get(id).is_some() {
                Ok(self.uris.get(id).unwrap())
            } else {
                Err(PSP34Error::TokenNotExists)
            }
        }
    }

    impl Psp34 for Item {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            self.collection_id
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
        fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
            self.approvals.get((owner, operator, &None)).is_some()
                || id.is_some() && self.approvals.get((owner, operator, id)).is_some()
            // todo!()
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
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let mut caller = self.env().caller();
            if let Some(id) = &id {
                let owner = self.owner_of(*id).ok_or(PSP34Error::TokenNotExists)?;
                if approved && owner == operator {
                    return Err(PSP34Error::SelfApprove);
                }

                if owner != caller && !self.allowance(owner, caller, None) {
                    return Err(PSP34Error::NotApproved);
                }

                if !approved && self.allowance(owner, operator, None) {
                    return Err(PSP34Error::Custom(String::from(
                    "Cannot revoke approval for a single token, when the operator has approval for all tokens."
                )));
                }
                caller = owner;
            }

            if approved {
                self.approvals.insert((caller, operator, id.as_ref()), &());
            } else {
                self.approvals.remove((caller, operator, id.as_ref()));
            }
            Ok(())
            // todo!()
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
    mod test {
        use franks_test_suite::unit_test;
        unit_test!(Item, Item::new);
    }
}
