use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, json_types::U128, NearToken, PublicKey};
use crate::Contract;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    fn get_test_public_key() -> PublicKey {
        // Use a valid ED25519 public key for testing
        PublicKey::from_str("ed25519:6E8sCci9badyRkXb3JoRpBj5p8C6Tw41ELDZoiihKEtp").unwrap()
    }

    #[test]
    fn test_new() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let contract = Contract::new(owner.clone(), U128(1_000_000));
        assert_eq!(contract.get_greeting(), "Hello");
        assert_eq!(contract.get_price().0, 1_000_000);
        assert_eq!(contract.get_sub_count(), 0);
    }

    #[test]
    fn test_greeting() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.set_greeting("howdy".to_string());
        assert_eq!(contract.get_greeting(), "howdy");
    }

    #[test]
    fn test_price_management() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let context = get_context(owner.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        assert_eq!(contract.get_price().0, 1_000_000);

        contract.set_price(U128(2_000_000));
        assert_eq!(contract.get_price().0, 2_000_000);
    }

    #[test]
    #[should_panic(expected = "Only the owner can set the price")]
    fn test_price_management_unauthorized() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let other: AccountId = "other.near".parse().unwrap();
        let context = get_context(other);
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.set_price(U128(2_000_000));
    }

    #[test]
    fn test_subaccount_management() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let user: AccountId = "user.near".parse().unwrap();
        let mut context = get_context(user.clone());
        // Set deposit to cover both price (1 NEAR) and storage (0.00125 NEAR)
        context.attached_deposit(NearToken::from_yoctonear(2_250_000_000_000_000_000_000));
        context.current_account_id("contract.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000_000_000_000_000_000));
        contract.user_create_sub_account("test".to_string(), get_test_public_key());

        assert_eq!(contract.get_sub_count(), 1);
        assert_eq!(contract.get_sub_addresses(0, 10), vec!["test.contract.near"]);
        // Check remaining balance after subtracting price and storage costs
        assert_eq!(contract.user_get_deposit_balance(user).0, 1250000000000000000000);
    }

    #[test]
    #[should_panic(expected = "Insufficient deposit for subaccount creation")]
    fn test_subaccount_creation_insufficient_funds() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let user: AccountId = "user.near".parse().unwrap();
        let mut context = get_context(user);
        context.attached_deposit(NearToken::from_yoctonear(500_000));
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.user_create_sub_account("test".to_string(), get_test_public_key());
    }
}
