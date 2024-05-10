type Id = u128;
use ink::prelude::vec::Vec;

#[ink::trait_definition]
pub trait Psp34Metadata {
    #[ink(message)]
    fn get_attribute(&self, id: Id, key: Vec<u8>) -> Option<Vec<u8>>;
}
