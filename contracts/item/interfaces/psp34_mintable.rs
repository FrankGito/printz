use crate::id::Id;
use crate::psp34_error::PSP34Error;

#[ink::trait_definition]
pub trait Psp34Mintable {
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP34Error>;
}
