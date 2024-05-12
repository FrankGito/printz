#[macro_export]
macro_rules! unit_test {
    ($contract:ident, $constructor:expr) => {
        mod psp34_unit_tests {
            use super::super::*;
            use ink::env::{test::*, DefaultEnvironment as E};
            #[ink::test]
            fn alice_has_zero() {
                // let mut item = $constructor(0, 100);
                let item = Item::new();
                let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                let alice = accounts.alice;
                let count = item.owned_tokens_count.get(alice);
                assert_eq!(count, None);
            }

            #[ink::test]
            fn balance_of_alice_is_zero() {
                // let mut item = $constructor(0, 100);
                let item = Item::new();
                let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                let alice = accounts.alice;
                let count = item.balance_of(alice);
                assert_eq!(count, 0u32);
            }

            #[ink::test]
            fn total_supply_is_zero() {
                // let mut item = $constructor(0, 100);
                let item = Item::new();
                let total_supply = item.total_supply();
                assert_eq!(total_supply, 0)
            }

            #[ink::test]
            fn mint_works() {
                let accounts = default_accounts::<E>();
                // Create a new contract instance.
                let mut token = $constructor();
                // Token 1 does not exists.
                assert_eq!(token.owner_of(1), None);
                // Alice does not owns tokens.
                assert_eq!(token.balance_of(accounts.alice), 0);
                // Create token Id 1.
                assert_eq!(token.mint(1), Ok(()));
                // Alice owns 1 token.
                assert_eq!(token.balance_of(accounts.alice), 1);
            }
            #[ink::test]
            fn mint_existing_should_fail() {
                let accounts = default_accounts::<E>();
                // Create a new contract instance.
                let mut token = $constructor();
                // Create token Id 1.
                assert_eq!(token.mint(1), Ok(()));
                // The first Transfer event takes place
                // assert_eq!(1, recorded_events().count());
                // Alice owns 1 token.
                assert_eq!(token.balance_of(accounts.alice), 1);
                // Alice owns token Id 1.
                assert_eq!(token.owner_of(1), Some(accounts.alice));
                // Cannot create  token Id if it exists.
                // Bob cannot own token Id 1.
                assert_eq!(token.mint(1), Err(PSP34Error::TokenExists));
            }

            #[ink::test]
            fn transfer_works() {
                let accounts = default_accounts::<E>();
                // Create a new contract instance.
                let mut token = $constructor();
                // Create token Id 1 for Alice
                assert_eq!(token.mint(1), Ok(()));
                // Alice owns token 1
                assert_eq!(token.balance_of(accounts.alice), 1);
                // Bob does not owns any token
                assert_eq!(token.balance_of(accounts.bob), 0);
                // The first Transfer event takes place
                // assert_eq!(1, recorded_events().count());
                // Alice transfers token 1 to Bob
                assert_eq!(token.transfer(accounts.bob, 1, Vec::new()), Ok(()));
                // The second Transfer event takes place
                // assert_eq!(2, recorded_events().count());
                // Bob owns token 1
                assert_eq!(token.balance_of(accounts.bob), 1);

                // let mut item = Item::new();
                // let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                // let alice = accounts.alice;
                // let bob = accounts.bob;
                // ink::env::test::set_caller::<ink::env::DefaultEnvironment>(alice);
                // let _ = item.mint(0);
                // let _ = item.transfer(bob, 1u128, Vec::new());
                // let count = item.owned_tokens_count.get(bob);
                // assert!(count.is_none())
            }
        }
    };
}
