#[macro_export]
macro_rules! unit_test {
    ($contract:ident, $constructor:expr) => {
        mod psp34_unit_tests {
            use super::super::*;
            use ink::env::test::set_caller;
            use ink::env::{test::*, DefaultEnvironment as E};

            #[ink::test]
            fn alice_has_zero() {
                let mut item = $constructor(0u128);
                let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                let alice = accounts.alice;
                let count = item.owned_tokens_count.get(alice);
                assert_eq!(count, None);
            }

            #[ink::test]
            fn balance_of_alice_is_zero() {
                let mut item = $constructor(0u128);
                let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
                let alice = accounts.alice;
                let count = item.balance_of(alice);
                assert_eq!(count, 0u32);
            }

            #[ink::test]
            fn total_supply_is_zero() {
                let mut item = $constructor(0u128);
                let total_supply = item.total_supply();
                assert_eq!(total_supply, 0)
            }

            #[ink::test]
            fn mint_works() {
                let accounts = default_accounts::<E>();
                // Create a new contract instance.
                let mut token = $constructor(0u128);
                // Token 1 does not exists.
                assert_eq!(token.owner_of(Id::U128(0)), None);
                // Alice does not owns tokens.
                assert_eq!(token.balance_of(accounts.alice), 0);
                // Create token Id 1.
                assert_eq!(token.mint(Id::U128(0)), Ok(()));
                // Alice owns 1 token.
                assert_eq!(token.balance_of(accounts.alice), 1);
            }

            #[ink::test]
            fn mint_existing_should_fail() {
                let accounts = default_accounts::<E>();
                // Create a new contract instance.
                let mut token = $constructor(0u128);
                // Create token Id 1.
                assert_eq!(token.mint(Id::U128(0)), Ok(()));
                // Alice owns 1 token.
                assert_eq!(token.balance_of(accounts.alice), 1);
                // Alice owns token Id 1.
                assert_eq!(token.owner_of(Id::U128(0)), Some(accounts.alice));
                // Bob cannot own token Id 1.
                assert_eq!(token.mint(Id::U128(0)), Err(PSP34Error::TokenExists));
            }

            #[ink::test]
            fn transfer_works() {
                let accounts = default_accounts::<E>();
                let mut token = $constructor(0u128);
                set_caller::<E>(accounts.alice);
                assert_eq!(token.mint(Id::U128(1)), Ok(()));
                // Alice owns token 1
                assert_eq!(token.balance_of(accounts.alice), 1u32);
                // Bob does not owns any token
                assert_eq!(token.balance_of(accounts.bob), 0u32);
                // Alice transfers token 1 to Bob
                assert_eq!(
                    token.transfer(accounts.bob, Id::U128(1), Vec::new()),
                    Ok(())
                );
                // Bob owns token 1
                assert_eq!(token.balance_of(accounts.bob), 1);
            }

            #[ink::test]
            fn allowance_works() {
                let accounts = default_accounts::<E>();
                let mut token = $constructor(0u128);
                //Alice mints Token 1
                set_caller::<E>(accounts.alice);
                assert_eq!(token.mint(Id::U128(1)), Ok(()));
                //Alice is owner
                assert_eq!(token.balance_of(accounts.alice), 1u32);
                //Alice has no allowance
                assert_eq!(
                    token.allowance(accounts.alice, accounts.alice, Some(Id::U128(1))),
                    false
                );
                //Bob has no allowance
                assert_eq!(
                    token.allowance(accounts.alice, accounts.bob, Some(Id::U128(1))),
                    false
                );
                //Alice approves Bob
                assert_eq!(token.approve(accounts.bob, Some(Id::U128(1)), true), Ok(()));
                //Bob has allowance
                assert_eq!(
                    token.allowance(accounts.alice, accounts.bob, Some(Id::U128(1))),
                    true
                );
            }
        }
    };
}
