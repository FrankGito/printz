use crate::id::Id;
use crate::psp34_error::PSP34Error;
use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

pub struct PSP34Item {
    contract_address: ink::contract_ref!(PSP34),
    item_id: Id,
}

pub struct PSP34ItemWithDescription {
    item: PSP34Item,
    description: Option<Vec<u8>>,
}

// TODO: events

#[ink::trait_definition]
pub trait PSP34Sale {

    #[ink(message)]
    fn seller(&self) -> AccountId;

    #[ink(message)]
    fn items_for_sale(&self) -> Vec<PSP34ItemWihDescription>;

    #[ink(message)]
    fn added_balance(&self) -> Balance;

    #[ink(message)]
    fn modify_sell(&self, add_assets: Vec<PSP34ItemWithDescription>, remove_assets: Vec<PSP34Item>, remove_value: Balance);

    #[ink(message, payable)]
    fn bid(&self, other_assets: Vec<PSP34ItemWithDescription>, message: Option<Vec<u8>>); // event

    #[ink(message, payable)]
    fn modify_bid(&self, add_assets: Vec<PSP34ItemWithDescription>, remove_assets: Vec<PSP34Item>, remove_value: Balance, message: Option<Vec<u8>>); // event

    #[ink(message)]
    fn accept_bid(&self, account: AccountId, message: Option<Vec<u8>>); // event

    #[ink(message)]
    fn reject_bid(&self, account: AccountId, message: Option<Vec<u8>>); // event

    #[ink(message)]
    fn revoke_bid(&self, message: Option<Vec<u8>>); // event

    #[ink(message)]
    fn close_sell(&self);

}

// Death Star model sale:
// bids:
// - from user 0xaa: Y-wing model, X-wing model + 12 Dots "please, please accept it"  [ Accept ] [ Reject ] [ Reply ]
// - from user 0xbb: 60 Dots                              "I don't care"              [ Accept ] [ Reject ] [ Reply ]

