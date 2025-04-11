
/// ❄️👋
/// Hello
/// sleet_subaccount_store_rust
/// methods
/// get_greeting
/// set_greeting



/// 👋
// Find all our documentation at https://docs.near.org
use near_sdk::{log, near, AccountId, Balance, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use std::collections::HashMap;






/// 👋
// Define the contract structure
#[near(contract_state)]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    greeting: String,
    price: Balance,
    owner_id: AccountId,
    subaccounts: UnorderedSet<String>,
    deposits: LookupMap<AccountId, Balance>,
}

// Define the default, which automatically initializes the contract
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, initial_price: U128) -> Self {
        Self {
            greeting: "Hello".to_string(),
            price: initial_price.0,
            owner_id,
            subaccounts: UnorderedSet::new(b"s"),
            deposits: LookupMap::new(b"d"),
        }
    }
}





/// 👋
// Implement the contract structure
#[near]
impl Contract {
    // Greeting methods
    pub fn get_greeting(&self) -> String {
        self.greeting.clone()
    }

    pub fn set_greeting(&mut self, greeting: String) {
        log!("Saving greeting: {greeting}");
        self.greeting = greeting;
    }

    // Price management methods
    pub fn set_price(&mut self, new_price: U128) {
        assert_eq!(
            self.owner_id,
            near_sdk::env::predecessor_account_id(),
            "Only the owner can set the price"
        );
        self.price = new_price.0;
    }

    pub fn get_price(&self) -> U128 {
        U128(self.price)
    }

    // Subaccount management methods
    pub fn get_sub_count(&self) -> u64 {
        self.subaccounts.len()
    }

    pub fn get_sub_addresses(&self, start_index: u64, limit: u64) -> Vec<String> {
        self.subaccounts
            .iter()
            .skip(start_index as usize)
            .take(limit as usize)
            .collect()
    }

    // User deposit and balance management
    #[payable]
    pub fn user_create_sub_account(&mut self, name: String) -> Promise {
        let deposit = near_sdk::env::attached_deposit();
        let account_id = near_sdk::env::predecessor_account_id();
        let current_balance = self.deposits.get(&account_id).unwrap_or(0);
        let new_balance = current_balance + deposit;
        assert!(new_balance >= self.price, "Insufficient deposit for subaccount creation");

        // Create the subaccount
        let subaccount_id = format!("{}.{}", name, near_sdk::env::current_account_id());
        assert!(!self.subaccounts.contains(&subaccount_id), "Subaccount already exists");
        
        // Update state
        self.subaccounts.insert(&subaccount_id);
        self.deposits.insert(&account_id, &(new_balance - self.price));

        // Return any excess deposit
        if deposit > self.price {
            Promise::new(account_id.clone()).transfer(deposit - self.price)
        } else {
            Promise::new(account_id)
        }
    }

    pub fn user_get_deposit_balance(&self, account_id: AccountId) -> U128 {
        U128(self.deposits.get(&account_id).unwrap_or(0))
    }

    #[payable]
    pub fn user_withdraw_balance(&mut self) -> Promise {
        let account_id = near_sdk::env::predecessor_account_id();
        let balance = self.deposits.get(&account_id).unwrap_or(0);
        assert!(balance > 0, "No balance to withdraw");

        self.deposits.remove(&account_id);
        Promise::new(account_id).transfer(balance)
    }
}


















///👋
/// TESTS !!!
#[cfg(test)]
mod unit_tests;
