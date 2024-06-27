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

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            // ACTION: `set` the total supply to `initial_supply`
            // ACTION: `insert` the `initial_supply` as the `caller` balance
            todo!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            // ACTION: Return the total supply
            todo!()
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: Return the balance of `owner`
            //   HINT: Use `balance_of_or_zero` to get the `owner` balance
            todo!()
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: `get` the balance of `owner`, then `unwrap_or` fallback to 0
            // ACTION: Return the balance
            todo!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            // ACTION: Call the `transfer_from_to` with `from` as `self.env().caller()`
            todo!()
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: Get the balance for `from` and `to`
            //   HINT: Use the `balance_of_or_zero` function to do this
            // ACTION: If `from_balance` is less than `value`, return `false`
            // ACTION: Insert new values for `from` and `to`
            //         * from_balance - value
            //         * to_balance + value
            // ACTION: Return `true`
            todo!()
        }
    }

    /// NOTE: Do not touch any below!
    /// These functions below are used to 
    /// test your code modification above
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::*;
        use ink::primitives::AccountId;

        type TestAccounts = test::DefaultAccounts<DefaultEnvironment>;
        type DefaultAccountsFunc = fn() -> TestAccounts;
        type AccountIdFunc = fn() -> AccountId;
        type SetCalleeFunc = fn(AccountId);

        const DEFAULT_ACCOUNTS: DefaultAccountsFunc = test::default_accounts::<DefaultEnvironment>;
        const ACCOUNT_ID: AccountIdFunc = account_id::<DefaultEnvironment>;
        const CALLEE: SetCalleeFunc = test::set_callee::<DefaultEnvironment>;
        const CALLER: SetCalleeFunc = test::set_caller::<DefaultEnvironment>;

        /// Instantiate contract using `new` constructor.
        /// Check the total supply if caller received
        /// the initial supply through `total_supply` function.
        #[ink::test]
        fn new_works() {
            // Constructor works
            let erc20 = Erc20::new(14);
            assert_eq!(erc20.total_supply(), 14);
        }

        /// Gets the balance of the contract instantiator.
        /// Must be equal to the initialized amount.
        /// Other accounts should have zero balance.
        #[ink::test]
        fn balance_works() {
            // Constructor works
            let erc20: Erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction
            let accounts: TestAccounts = DEFAULT_ACCOUNTS();

            // Alice owns all the tokens on contract instantiation
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            // Bob does not owns tokens
            assert_eq!(erc20.balance_of(accounts.bob), 0);
        }

        /// Transfer the balance from one account to another.
        /// Transfer function should return `true` value.
        /// If transfer function returns `true`, receiver balance
        /// is checked if the transfer was successful.
        #[ink::test]
        fn transfer_works() {
            // Constructor works.
            let mut erc20: Erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts: TestAccounts = DEFAULT_ACCOUNTS();

            assert_eq!(erc20.balance_of(accounts.bob), 0);
            // Alice transfers 10 tokens to Bob.
            assert_eq!(erc20.transfer(accounts.bob, 10), true);
            // Bob owns 10 tokens.
            assert_eq!(erc20.balance_of(accounts.bob), 10);
        }

        /// Transfer function is called here and must fail.
        /// It checks if the function will return false.
        /// Get the balance of the accounts to check
        /// balance wasn't transferred as it was failed.
        #[ink::test]
        fn invalid_transfer_should_fail() {
            // Constructor works.
            let mut erc20: Erc20 = Erc20::new(100);
            let accounts: TestAccounts = DEFAULT_ACCOUNTS();

            assert_eq!(erc20.balance_of(accounts.bob), 0);

            // Set the contract as callee and Bob as caller.
            let contract: AccountId = ACCOUNT_ID();
            CALLEE(contract);
            CALLER(accounts.bob);

            // Bob fails to transfers 10 tokens to Eve.
            assert_eq!(
                erc20.transfer(accounts.eve, 10),
                false
            );
            // Alice owns all the tokens.
            assert_eq!(erc20.balance_of(accounts.alice), 100);
            assert_eq!(erc20.balance_of(accounts.bob), 0);
            assert_eq!(erc20.balance_of(accounts.eve), 0);
        }
    }
}
