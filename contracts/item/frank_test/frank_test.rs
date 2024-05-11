#[macro_export]
macro_rules! frank_test {
    ($contract:ident, $constructor:expr) => {
        mod psp34_unit_tests {
            use super::super::*;
            use ink::env::{test::*, DefaultEnvironment as E};

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
        }
    };
}
