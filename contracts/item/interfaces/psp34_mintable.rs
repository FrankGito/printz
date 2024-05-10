#![cfg_attr(not(feature = "std"), no_std, no_main)]

type Id = u128;

#[ink::trait_definition]
pub trait Psp34Mintable {
    #[ink(message)]
    fn mint(&mut self, id: Id) -> Result<(), PSP34Error>;
}
