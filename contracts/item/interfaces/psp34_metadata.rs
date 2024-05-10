#![cfg_attr(not(feature = "std"), no_std, no_main)]

type Id = u128;

#[ink::trait_definition]
pub trait Psp34Metadata {
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}
