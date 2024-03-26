#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        /// The total supply.
        total_supply: Balance,
        /// The balance of each user.
        balances: Mapping<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
    //  ACTION: Create a `Transfer` event with:
    //          * from: Option<AccountId>
    //          * to: Option<AccountId>
    //          * value: Balance
    }

    // ACTION: Add an `Approval` event
	//         It should emit the following:
	//         * `owner` as an `AccountId`
	//         * `spender` as an `AccountId`
	//         * `value` as a `Balance`

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!

            // ACTION: Call `Self::env().emit_event` with the `Transfer` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
            todo!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!
            todo!()
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!
            todo!()
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!
            todo!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!
            todo!()
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: After creating an ERC20,
            //          * copy and paste your source code
            //          * from the ERC20 `lib.rs` and
            //          * you can now define your event!
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink::primitives::{
            Clear,
            Hash,
        };

        #[ink::test]
        fn new_works() {
            let contract = Erc20::new(777);
            assert_eq!(contract.total_supply(), 777);
        }

        #[ink::test]
        fn balance_works() {
            let contract = Erc20::new(100);
            assert_eq!(contract.total_supply(), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut contract = Erc20::new(100);
            assert_eq!(contract.balance_of(AccountId::from([0x1; 32])), 100);
            assert!(contract.transfer(AccountId::from([0x0; 32]), 10));
            assert_eq!(contract.balance_of(AccountId::from([0x0; 32])), 10);
            assert!(!contract.transfer(AccountId::from([0x0; 32]), 100));
        }
    }
}