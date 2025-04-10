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
near deploy <your-account>.testnet build_near/sleet_rust_hello.wasm

# For mainnet deployment
near deploy <your-account>.near build_near/sleet_rust_hello.wasm
```

---


### Methods

- get_greeting
- set_greeting

- set_price, for setting the required near deposit for sub account
- get_price, for getting the required price
- get_deposit, for getting their deposit baance
- get_sub_count, for getting count of subaccounts
- get_sub_adresses,  for getting a list of of subaccounts
- create_sub_account, for creating a subaccount, anyone can call this if they have deposied the min required




---

copyright 2025 by sleet.near
