# Gyde Multi-User NFT Smart Contract

This smart contract allows creating four types of NFTs representing users:
* User
* Super User
* Organization
* Admin

Only people owning Admin NFTs and the smart contract owner can mint NFTs of any of the other types.

NFTs are **non-transferable**. Only Admins can transfer them.

## Installation and Deployment

1. Install Rust.
2. Add `wasm32-unknown-unknown` by running
```rust
rustup target add wasm32-unknown-unknown
```
3. Run `./build.sh` in order to create the WASM compiled smart contract.
4. In order to deploy the Smart Contract into your Near account, run the following:
```bash
export NEAR_ENV=testnet # Make sure to select the correct network for your usage. testnet is for testing, mainnet is the live environment
export NEAR_WALLET=<your_wallet_id>
near deploy $NEAR_WALLET ./out/main.wasm --accountId $NEAR_WALLET # deploy the smart contract on the account
near call $NEAR_WALLET new_default_meta '{"owner_id": "'$NEAR_WALLET'"}' --accountId $NEAR_WALLET # initialize the smart contract with default data
```

## Contract functions
 TBD
 
## FAQ
