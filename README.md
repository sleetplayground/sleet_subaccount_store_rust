# sleet_subaccount_store_rust

📝 turns any near account into a subaccount store

---

### Build and Test

```bash
# CARGO COMMANDS
cargo check
cargo test
cargo clean

# CUSTOM SCRIPTS
./build_cargo.sh
./build_reproducible.sh
./clean.sh # Just cleans the .wasm files and the custom build directories
```

DOCKER
<br/>
check docker for latest image and to pull
<br/>
https://hub.docker.com/r/sourcescan/cargo-near/tags


###  How to Deploy?

```bash
# Deploy to testnet
near deploy <your-account>.testnet build_near/sleet_subaccount_store_rust.wasm

# For mainnet deployment
near deploy <your-account>.near build_near/sleet_subaccount_store_rust.wasm
```

---

### Methods

- get_greeting
- set_greeting
To initialize the contract, you need to specify the owner account and initial price for subaccounts:

- set_price, for setting the required near deposit for sub account
- get_price, for getting the required price
- get_sub_count, for getting count of subaccounts
- get_sub_addresses, for getting a list of subaccounts (takes start_index and limit as parameters)
- user_get_deposit_balance, for getting their deposit balance
- user_create_sub_account, for creating a subaccount, anyone can call this if they have deposied the min required
- user_withdraw_balance, so a user can withdraw their deposit balance
- contract should keep track of users deposit balances whenever anyone trasfers near to the contract
- admin_add_account, for adding subaccounts to the list, users should not be able to create subaccounts for accounts that already exist and are on the list
- admin_remove_account, for removing subaccounts from the list.

---


### Contract Initialization

To initialize the contract, you need to specify the owner account and initial price for subaccounts:

```bash
# Initialize on testnet (price in yoctoNEAR, 1 N = 1e24 yN)
near call <your-account>.testnet new '{"owner_id": "<your-account>.testnet", "initial_price": "1000000000000000000000000"}' --accountId <your-account>.testnet

# Initialize on mainnet
near call <your-account>.near new '{"owner_id": "<your-account>.near", "initial_price": "1000000000000000000000000"}' --accountId <your-account>.near
```

### Contract Methods

#### Price Management
```bash
# Get current price
near view <contract>.testnet get_price

# Set new price (owner only)
near call <contract>.testnet set_price '{"new_price": "2000000000000000000000000"}' --accountId <owner>.testnet
```

#### Subaccount Management
```bash
# Create a subaccount (requires attached deposit >= price + 0.00125 NEAR for storage)
# Note: Replace <public-key> with your actual public key in Base58 format
near call <contract>.testnet user_create_sub_account '{"name": "mysubaccount", "new_public_key": "<public-key>"}' --deposit 1.1 --accountId <your-account>.testnet

# Get total subaccount count
near view <contract>.testnet get_sub_count

# List subaccounts (paginated)
near view <contract>.testnet get_sub_addresses '{"start_index": 0, "limit": 10}'
```

#### Balance Management
```bash
# Check your deposit balance
near view <contract>.testnet user_get_deposit_balance '{"account_id": "<your-account>.testnet"}'

# Withdraw your balance
near call <contract>.testnet user_withdraw_balance '{}' --accountId <your-account>.testnet
```

### Notes
- All prices and deposits are in yoctoNEAR (1 N = 1e24 yN)
- Subaccount creation requires a deposit greater than or equal to the current price
- Excess deposits are automatically refunded
- The contract tracks user deposit balances for future subaccount creation

---

copyright 2025 by sleet.near
