# Gyde Multi-User NFT Smart Contract

This smart contract allows creating four types of NFTs representing users:
* User
* Super User
* Organization
* Admin

Only people owning Admin NFTs and the smart contract owner can mint NFTs of any of the other types.

NFTs are **non-transferable**. Only Admins can transfer them.

## Installation and Deployment

1. Install  Rust - https://www.rust-lang.org/tools/install.
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
 
 The main functions of this contract are:
 * **nft_mint** - allows creating a token with custom metadata
   * Usage example:
   ```bash
    near call <CONTRACT_ID> nft_mint \
    '{
        "token_id": <UNIQUE_NFT_ID>
        "metadata": {
                "expires_at": <>, 
                "extra": <>, 
                "issued_at": <>, 
                "copies": <>,
                "media_hash": <>,
                "reference": <>,
                "reference_hash": <>,
                "starts_at": <>,
                "updated_at": <>,
                "title": "First NFT",
                "description": "Cool description",
                "media": "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse3.mm.bing.net%2Fth%3Fid%3DOIP.Fhp4lHufCdTzTeGCAblOdgHaF7%26pid%3DApi&f=1",
                "user_type": "Admin", # string, accepts user/super_user/admin
                "organization": String # ex. "Gyde" or "Food Bank"
            }
        "receiver_id": <RECEIVER_ACCOUNT_ID> 
    }' \
    --accountId <ADMIN_ACCOUNT_ID>
    --deposit 1
   ```
 * **nft_modify_token** - allows modifying token metadata
   * structure is the same as above, but receiver ID is not needed.
   * every field of the NFT metadata can be changed

## Constraints and Exceptions

- If NFTs receive user types as metadata, those can be only Admin(admin), Super User(super_user) and User(user). If another is provided, an error is raised.
- NFT_TRANSFER function works only if the caller has an Admin NFT in his possession. Else, an error is raised and the transaction fails.
- Contract owner can call any function without having an Admin NFT.
