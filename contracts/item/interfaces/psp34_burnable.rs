use crate::{id::Id, psp34_error::PSP34Error};
use ink::primitives::AccountId;

#[ink::trait_definition]
pub trait PSP34Enumerable {
    #[ink(message)]
    fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error>;
}
