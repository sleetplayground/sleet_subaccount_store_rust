use near_sdk::test_utils::VMContextBuilder;
use near_sdk::{testing_env, AccountId, json_types::U128};
use crate::Contract;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
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
        let mut context = get_context(owner.clone());
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
        let mut context = get_context(other);
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.set_price(U128(2_000_000));
    }

    #[test]
    fn test_subaccount_management() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let user: AccountId = "user.near".parse().unwrap();
        let mut context = get_context(user.clone());
        context.attached_deposit(2_000_000.into());
        context.current_account_id("contract.near".parse().unwrap());
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.user_create_sub_account("test".to_string());

        assert_eq!(contract.get_sub_count(), 1);
        assert_eq!(contract.get_sub_addresses(0, 10), vec!["test.contract.near"]);
        assert_eq!(contract.user_get_deposit_balance(user).0, 1_000_000);
    }

    #[test]
    #[should_panic(expected = "Insufficient deposit for subaccount creation")]
    fn test_subaccount_creation_insufficient_funds() {
        let owner: AccountId = "owner.near".parse().unwrap();
        let user: AccountId = "user.near".parse().unwrap();
        let mut context = get_context(user);
        context.attached_deposit(500_000.into());
        testing_env!(context.build());

        let mut contract = Contract::new(owner, U128(1_000_000));
        contract.user_create_sub_account("test".to_string());
    }
}
