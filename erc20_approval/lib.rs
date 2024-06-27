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
        // Approval spender on behalf of the message's sender.
		//  ACTION: Add an `allowances` storage item. It should be a
		//         `HashMap` from `(AccountId, AccountId)` to `Balance`
    }

    #[ink(event)]
    pub struct Transfer {
	    // ACTION: After creating an ERC20 EVENT,
	    //          * copy and paste your source code
	    //          * from the ERC20 EVENT `lib.rs` and
	    //          * you can now define your new event
	    //          * and create an approval function!
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

            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        fn balance_of_or_zero(&self, owner: &AccountId) -> Balance {
            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
            // ACTION: After creating an ERC20 EVENT,
            //          * copy and paste your source code
            //          * from the ERC20 EVENT `lib.rs` and
            //          * you can now define your new event
            //          * and create an approval function!
            todo!()
        }

        #[ink(message)]
		pub fn approve(&mut self, spender: AccountId, value: Balance) -> bool {
			// ACTION: Get the `self.env().caller()` and store it as the `owner`
			// ACTION: Insert the new allowance into the `allowances` HashMap
			//   HINT: The key tuple is `(owner, spender)`
			// ACTION: `emit` the `Approval` event you created using these values
			// ACTION: Return true if everything was successful
            todo!()
		}

        #[ink(message)]
		pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
			// ACTION: Create a getter for the `allowances` HashMap
			//   HINT: Take a look at the getters above if you forget the details
			// ACTION: Return the `allowance` value
            todo!()
		}

        #[ink(message)]
		pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> bool {
			// ACTION: Get the allowance for `(from, self.env().caller())` using `allowance_of_or_zero`
			// ACTION: `if` the `allowance` is less than the `value`, exit early and return `false`
			// ACTION: `insert` the new allowance into the map for `(from, self.env().caller())`
			// ACTION: Finally, call the `transfer_from_to` for `from` and `to`
			// ACTION: Return true if everything was successful
            todo!()
		}

        fn allowance_of_or_zero(&self, owner: &AccountId, spender: &AccountId) -> Balance {
			// ACTION: `get` the `allowances` of `(owner, spender)` and `unwrap_or` return `0`.
            todo!()
		}
    }

    /// NOTE: Do not touch any below!
    /// These functions below are used to 
    /// test your code modification above
    #[cfg(test)]
    mod tests {
        use scale_info::scale;
        use super::*;
        use ink::env::*;
        use ink::primitives::AccountId;
        use ink::primitives::{
            Clear,
            Hash,
        };

        type TestAccounts = test::DefaultAccounts<DefaultEnvironment>;
        type EmittedEvents = Vec<test::EmittedEvent>;
        type DefaultAccountsFunc = fn() -> TestAccounts;
        type AccountIdFunc = fn() -> AccountId;
        type SetCalleeFunc = fn(AccountId);
        
        const DEFAULT_ACCOUNTS: DefaultAccountsFunc = test::default_accounts::<DefaultEnvironment>;
        const ACCOUNT_ID: AccountIdFunc = account_id::<DefaultEnvironment>;
        const CALLEE: SetCalleeFunc = test::set_callee::<DefaultEnvironment>;
        const CALLER: SetCalleeFunc = test::set_caller::<DefaultEnvironment>;

        fn assert_transfer_event(
            event: &test::EmittedEvent,
            expected_from: Option<AccountId>,
            expected_to: Option<AccountId>,
            expected_value: Balance,
        ) {
            let decoded_event =
                <Transfer as scale::Decode>::decode(&mut &event.data[..])
                    .expect("encountered invalid contract event data buffer");
                
            let Transfer { from, to, value } = decoded_event;
            assert_eq!(from, expected_from, "encountered invalid Transfer.from");
            assert_eq!(to, expected_to, "encountered invalid Transfer.to"); // error on instantiating contract transfer event
            assert_eq!(value, expected_value, "encountered invalid Trasfer.value");

            let mut expected_topics: Vec<Hash> = Vec::new();
            expected_topics.push(
                ink::blake2x256!("Transfer(Option<AccountId>,Option<AccountId>,Balance)")
                    .into(),
            );
            if let Some(from) = expected_from {
                expected_topics.push(encoded_into_hash(from));
            } else {
                expected_topics.push(Hash::CLEAR_HASH);
            }
            if let Some(to) = expected_to {
                expected_topics.push(encoded_into_hash(to));
            } else {
                expected_topics.push(Hash::CLEAR_HASH);
            }
            expected_topics.push(encoded_into_hash(value));

            let topics: Vec<Vec<u8>> = event.topics.clone();
            for (n, (actual_topic, expected_topic)) in
                topics.iter().zip(expected_topics).enumerate()
            {
                let mut topic_hash: Hash = Hash::CLEAR_HASH;
                let len: usize = actual_topic.len();
                topic_hash.as_mut()[0..len].copy_from_slice(&actual_topic[0..len]);

                assert_eq!(
                    topic_hash, expected_topic,
                    "encountered invalid topic at {n}"
                );
            }
        }

        fn encoded_into_hash<T>(entity: T) -> Hash
        where
            T: scale::Encode,
        {
            use ink::{
                env::hash::{
                    Blake2x256,
                    CryptoHash,
                    HashOutput,
                },
                primitives::Clear,
            };

            let mut result: Hash = Hash::CLEAR_HASH;
            let len_result: usize = result.as_ref().len();
            let encoded: Vec<u8> = entity.encode();
            let len_encoded: usize = encoded.len();
            if len_encoded <= len_result {
                result.as_mut()[..len_encoded].copy_from_slice(&encoded);
                return result
            }
            let mut hash_output: [u8; 32] =
                <<Blake2x256 as HashOutput>::Type as Default>::default();
            <Blake2x256 as CryptoHash>::hash(&encoded, &mut hash_output);
            let copy_len: usize = core::cmp::min(hash_output.len(), len_result);
            result.as_mut()[0..copy_len].copy_from_slice(&hash_output[0..copy_len]);
            result
        }

        /// Instantiate contract using `new` constructor.
        /// Check the total supply if caller received
        /// the initial supply through `total_supply` function.
        /// 
        /// It includes event checking to see if it
        /// registered successfully on the events.
        #[ink::test]
        fn new_works() {
            // Constructor works.
            let _erc20: Erc20 = Erc20::new(100);

            // Transfer event triggered during initial construction.
            let emitted_events: EmittedEvents = test::recorded_events().collect();
            assert_eq!(1, emitted_events.len());

            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// Gets the balance of the contract instantiator.
        /// Must be equal to the initialized amount.
        /// Other accounts should have zero balance.
        /// 
        /// It includes event checking to see if it
        /// registered successfully on the events.
        #[ink::test]
        fn balance_of_works() {
            // Constructor works
            let erc20: Erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction
            let emitted_events: EmittedEvents = test::recorded_events().collect();
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
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
        /// 
        /// It includes event checking to see if it
        /// registered successfully on the events.
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

            let emitted_events: EmittedEvents = test::recorded_events().collect();
            assert_eq!(emitted_events.len(), 2);
            // Check first transfer event related to ERC-20 instantiation.
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // Check the second transfer event relating to the actual trasfer.
            assert_transfer_event(
                &emitted_events[1],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x02; 32])),
                10,
            );
        }

        /// Transfer function is called here and must fail.
        /// It checks if the function will return false.
        /// Get the balance of the accounts to check
        /// balance wasn't transferred as it was failed.
        /// 
        /// It includes event checking to see if it
        /// registered successfully on the events.
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

            // Transfer event triggered during initial construction.
            let emitted_events: EmittedEvents = test::recorded_events().collect();
            assert_eq!(emitted_events.len(), 1);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
        }

        /// This will call the `approve` function which
        /// will allow sender to send balance to another
        /// on their behalf.
        /// 
        /// Calls `transfer_from` later if the sender 
        /// approved the token to be sent.
        /// 
        /// If transfer function returns `true`, receiver balance
        /// is checked if the transfer was successful.
        /// 
        /// It includes event checking to see if it
        /// registered successfully on the events.
        #[ink::test]
        fn transfer_from_works() {
            // Constructor works.
            let mut erc20: Erc20 = Erc20::new(100);
            // Transfer event triggered during initial construction.
            let accounts: TestAccounts = DEFAULT_ACCOUNTS();

            // Bob fails to transfer tokens owned by Alice.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                false
            );
            // Alice approves Bob for token transfers on her behalf.
            assert_eq!(erc20.approve(accounts.bob, 10), true);

            // The approve event takes place.
            assert_eq!(test::recorded_events().count(), 2);

            // Set the contract as callee and Bob as caller.
            let contract: AccountId = ACCOUNT_ID();
            CALLEE(contract);
            CALLER(accounts.bob);

            // Bob transfers tokens from Alice to Eve.
            assert_eq!(
                erc20.transfer_from(accounts.alice, accounts.eve, 10),
                true
            );
            // Eve owns tokens.
            assert_eq!(erc20.balance_of(accounts.eve), 10);

            // Check all transfer events that happened during the previous calls:
            let emitted_events: EmittedEvents = test::recorded_events().collect();
            assert_eq!(emitted_events.len(), 3);
            assert_transfer_event(
                &emitted_events[0],
                None,
                Some(AccountId::from([0x01; 32])),
                100,
            );
            // The second event `emitted_events[1]` is an Approve event that we skip
            // checking.
            assert_transfer_event(
                &emitted_events[2],
                Some(AccountId::from([0x01; 32])),
                Some(AccountId::from([0x05; 32])),
                10,
            );
        }

        // This will not work as the contract doesn't 
        // have error handling enumeration 
        // #[ink::test]
        // fn allowance_must_not_change_on_failed_transfer() {
        //     let mut erc20 = Erc20::new(100);
        //     let accounts: TestAccounts = DEFAULT_ACCOUNTS();

        //     // Alice approves Bob for token transfers on her behalf.
        //     let alice_balance = erc20.balance_of(accounts.alice);
        //     let initial_allowance = alice_balance + 2;
        //     assert_eq!(erc20.approve(accounts.bob, initial_allowance), true);

        //     // Get contract address.
        //     let contract: AccountId = ACCOUNT_ID();
        //     CALLEE(callee);
        //     CALLER(accounts.bob);

        //     // Bob tries to transfer tokens from Alice to Eve.
        //     let emitted_events_before = EmittedEvents = test::recorded_events().count();
        //     assert_eq!(
        //         erc20.transfer_from(accounts.alice, accounts.eve, alice_balance + 1),
        //         false
        //     );
        //     // Allowance must have stayed the same
        //     assert_eq!(
        //         erc20.allowance(accounts.alice, accounts.bob),
        //         initial_allowance
        //     );
        //     // No more events must have been emitted
        //     assert_eq!(
        //         emitted_events_before,
        //         test::recorded_events().count()
        //     )
        // }
    }
}
