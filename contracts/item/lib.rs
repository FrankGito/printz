#![cfg_attr(not(feature = "std"), no_std, no_main)]

#![allow(unexpected_cfgs)]
#![allow(non_local_definitions)]
#![allow(clippy::type_complexity)]

#[ink::contract]
pub mod item {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;
    use interfaces::id::Id;
    use interfaces::psp34::PSP34;
    use interfaces::psp34_error::PSP34Error;
    use interfaces::psp34_metadata::PSP34Metadata;
    use interfaces::psp34_mintable::PSP34Mintable;

    #[ink(storage)]
    pub struct Item {
        owned_tokens_count: Mapping<AccountId, u32>,
        token_owner: Mapping<Id, AccountId>,
        total_supply: u128,
        approvals: Mapping<(AccountId, AccountId), Option<Id>>,
        attributes: Mapping<(Id, Vec<u8>), Vec<u8>>,
    }

    #[ink(event)]
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        id: Id,
    }
    #[ink(event)]
    pub struct Approval {
        owner: AccountId,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    }

    #[ink(event)]
    pub struct AttributeSet {
        id: Id,
        key: Vec<u8>,
        data: Vec<u8>,
    }

    impl Item {
        #[ink(constructor)]
        pub fn new(total_supply: u128) -> Self {
            Self {
                owned_tokens_count: Mapping::new(),
                total_supply,
                token_owner: Mapping::new(),
                approvals: Mapping::new(),
                attributes: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn set_attribute(&mut self, id: Id, key: Vec<u8>, value: Vec<u8>) -> Result<(), PSP34Error> {
            if !self.token_owner.contains(&id) {
                self.attributes.insert((&id, &key), &value);
                Self::env().emit_event(AttributeSet { id, key, data: value });
                Ok(())
            }
            else {
                Err(PSP34Error::Custom(String::from("Token already minted")))
            }
        }
    }

    impl PSP34 for Item {
        #[ink(message)]
        fn collection_id(&self) -> Id {
            let account_id: AccountId = self.env().caller();
            Id::Bytes(<_ as AsRef<[u8; 32]>>::as_ref(&account_id).to_vec())
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
            self.approvals.get((owner, operator)).map(
                |allowance: Option<Id>| allowance.is_none() || allowance == id
            ).unwrap_or(false)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, id: Id, _data: Vec<u8>) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            self.token_owner.get(&id).map(|owner| {
                if owner != caller && !self.allowance(owner, caller, Some(id.clone())) {
                    return Err(PSP34Error::NotApproved)
                }
                let new_count_owner = self.owned_tokens_count.get(owner).and_then(|x| x.checked_sub(1)).filter(|x| x > &0u32).unwrap_or(0u32);
                self.owned_tokens_count.insert(owner, &new_count_owner);
                self.token_owner.insert(id.clone(), &to);
                let new_count_to = self.owned_tokens_count.get(to).and_then(|x| x.checked_add(1)).filter(|x| x > &0u32).unwrap_or(1u32);
                self.owned_tokens_count.insert(to, &new_count_to);

                if self.approvals.get((owner, caller)) == Some(Some(id.clone())) {
                    self.approvals.remove((owner, caller));
                }

                Self::env().emit_event(Transfer {
                    from: Some(caller),
                    to: Some(to),
                    id,
                });
                Ok(())
            }).unwrap_or(Err(PSP34Error::TokenNotExists))
        }

        #[ink(message)]
        fn approve(
            &mut self,
            operator: AccountId,
            id: Option<Id>,
            approved: bool,
        ) -> Result<(), PSP34Error> {
            let mut caller: AccountId = self.env().caller();
            if let Some(id) = id.clone() {
                let owner = self
                    .owner_of(id)
                    .ok_or(PSP34Error::TokenNotExists)?;
                if approved && owner == operator {
                    return Err(PSP34Error::SelfApprove);
                }

                if owner != caller && !self.allowance(owner, caller, None) {
                    return Err(PSP34Error::NotApproved);
                }

                if !approved && self.allowance(owner, operator, None) {
                    return Err(PSP34Error::Custom(String::from(
                      "Cannot revoke approval for a single token, when the operator has approval for all tokens." )));
                }
                caller = owner;
            }

            if approved {
                self.approvals.insert((caller, operator), &id);
            } else {
                self.approvals.remove((caller, operator));
            }

            Self::env().emit_event(Approval {
                owner: caller,
                operator,
                id,
                approved,
            });

            Ok(())
        }

        #[ink(message)]
        fn owner_of(&self, id: Id) -> Option<AccountId> {
            self.token_owner.get(id)
        }
    }

    impl PSP34Mintable for Item {
        #[ink(message)]
        fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
            // Check if Id is already taken
            if self.token_owner.get(id.clone()).is_some() {
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

    impl PSP34Metadata for Item {
        #[ink(message)]
        fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>> {
            self.attributes.get((&id, &key))
        }
    }

    #[cfg(test)]
    mod test {
        use franks_test_suite::unit_test;
        unit_test!(Item, Item::new);
    }
}
