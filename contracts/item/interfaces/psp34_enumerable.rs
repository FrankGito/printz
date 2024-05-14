use crate::{id::Id, psp34_error::PSP34Error};
use ink::primitives::AccountId;

#[ink::trait_definition]
pub trait PSP34Enumerable {
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, PSP34Error>;

    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Result<Id, PSP34Error>;
}
