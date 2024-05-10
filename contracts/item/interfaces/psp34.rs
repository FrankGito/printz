#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::primitives::AccountId;
type Id = u128;

#[ink::trait_definition]
pub trait Psp34 {
    #[ink(message)]
    fn collection_id(&self) -> Id;

    #[ink(message)]
    fn total_supply(&self) -> u128;

    #[ink(message)]
    fn balance_of(&self, owner: AccountId) -> u32;

    #[ink(message)]
    fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool;

    #[ink(message)]
    fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn approve(
        &mut self,
        operator: AccountId,
        id: Option<Id>,
        approved: bool,
    ) -> Result<(), PSP34Error>;

    #[ink(message)]
    fn owner_of(&self, id: Id) -> Option<AccountId>;
}
