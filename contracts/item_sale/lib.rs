#![cfg_attr(not(feature = "std"), no_std, no_main)]

#![allow(unexpected_cfgs)]
#![allow(non_local_definitions)]
#![allow(clippy::type_complexity)]

#[ink::contract]
pub mod item_sale {

    #[ink(storage)]
    pub struct ItemSale {
        seller: AccountId
    }

    impl ItemSale {

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                seller: Self::env().caller()
            }
        }

        #[ink(message)]
        pub fn get_seller(&self) -> AccountId {
            self.seller
        }

    }

    // TODO: implement me
    // impl PSP34Sale for ItemSale {
    // }

    // TODO: remove me
    impl Default for ItemSale {
        fn default() -> Self {
            Self::new()
        }
    }

}
