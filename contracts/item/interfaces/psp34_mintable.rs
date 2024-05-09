#![cfg_attr(not(feature = "std"), no_std, no_main)]

type Id = u128;

#[ink::trait_definition]
pub trait psp34_metadata {
    #[ink(message)]
    fn mint(id: Id) -> Result<(), PSP34Error>;
}
