
/// ❄️👋
/// Hello
/// sleet_subaccount_store_rust
/// Contract Methods:
/// - get_greeting() -> String
/// - set_greeting(greeting: String)
/// - set_price(new_price: U128)
/// - get_price() -> U128
/// - get_sub_count() -> u64
/// - get_sub_addresses(start_index: u64, limit: u64) -> Vec<String>
/// - user_create_sub_account(name: String) -> Promise
/// - user_get_deposit_balance(account_id: AccountId) -> U128
/// - user_withdraw_balance() -> Promise



/// 👋
// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::{log, near_bindgen, AccountId, NearToken, Promise, PublicKey};
use near_sdk::{env, PanicOnDefault};
use near_sdk::json_types::Base58PublicKey;



/// 👋
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    greeting: String,
    price: NearToken,
    owner_id: AccountId,
    subaccounts: UnorderedSet<String>,
    deposits: LookupMap<AccountId, NearToken>,
}

// Define the default, which automatically initializes the contract

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, initial_price: U128) -> Self {
        Self {
            greeting: "Hello".to_string(),
            price: NearToken::from_yoctonear(initial_price.0),
            owner_id,
            subaccounts: UnorderedSet::new(b"s"),
            deposits: LookupMap::new(b"d"),
        }
    }
}





/// 👋
// Implement the contract structure
#[near_bindgen]
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
        self.price = NearToken::from_yoctonear(new_price.0);
    }

    pub fn get_price(&self) -> U128 {
        U128(self.price.as_yoctonear())
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
    pub fn user_create_sub_account(&mut self, name: String, new_public_key: Base58PublicKey) -> Promise {
        let deposit = env::attached_deposit();
        let account_id = env::predecessor_account_id();
        let current_balance = self.deposits.get(&account_id).unwrap_or(NearToken::from_yoctonear(0));
        let new_balance = NearToken::from_yoctonear(current_balance.as_yoctonear() + deposit.as_yoctonear());
        assert!(new_balance >= self.price, "Insufficient deposit for subaccount creation");

        // Create the subaccount
        let subaccount_id = format!("{}.{}", name, env::current_account_id());
        let subaccount = AccountId::new_unchecked(subaccount_id.clone());
        assert!(!self.subaccounts.contains(&subaccount_id), "Subaccount already exists");
        
        // Update state
        self.subaccounts.insert(&subaccount_id);
        self.deposits.insert(&account_id, &(NearToken::from_yoctonear(new_balance.as_yoctonear() - self.price.as_yoctonear())));

        // Calculate the required amount for account creation (0.00125 NEAR for storage)
        let required_balance = NearToken::from_near(0.00125);

        // Create the new account and add full access key
        Promise::new(subaccount)
            .create_account()
            .add_full_access_key(new_public_key.into())
            .transfer(required_balance)
            .then(
                Promise::new(account_id)
                    .transfer(NearToken::from_yoctonear(deposit.as_yoctonear() - self.price.as_yoctonear() - required_balance.as_yoctonear()))
            )
    }

    pub fn user_get_deposit_balance(&self, account_id: AccountId) -> U128 {
        U128(self.deposits.get(&account_id).unwrap_or(NearToken::from_yoctonear(0)).as_yoctonear())
    }

    #[payable]
    pub fn user_withdraw_balance(&mut self) -> Promise {
        let account_id = near_sdk::env::predecessor_account_id();
        let balance = self.deposits.get(&account_id).unwrap_or(NearToken::from_yoctonear(0));
        assert!(balance > NearToken::from_yoctonear(0), "No balance to withdraw");

        self.deposits.remove(&account_id);
        Promise::new(account_id).transfer(balance)
    }

    // Admin account management methods
    pub fn admin_add_account(&mut self, subaccount_id: String) {
        assert_eq!(
            self.owner_id,
            near_sdk::env::predecessor_account_id(),
            "Only the owner can add accounts"
        );
        assert!(!self.subaccounts.contains(&subaccount_id), "Subaccount already exists");
        self.subaccounts.insert(&subaccount_id);
    }

    pub fn admin_remove_account(&mut self, subaccount_id: String) {
        assert_eq!(
            self.owner_id,
            near_sdk::env::predecessor_account_id(),
            "Only the owner can remove accounts"
        );
        assert!(self.subaccounts.contains(&subaccount_id), "Subaccount does not exist");
        self.subaccounts.remove(&subaccount_id);
    }
}


















///👋
/// TESTS !!!
#[cfg(test)]
mod unit_tests;
