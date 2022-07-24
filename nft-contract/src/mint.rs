use crate::*;

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();
        let mut metadata = metadata;

        assert!(
            self.verify_admin_token(&env::predecessor_account_id()),
            "Caller is not an Admin."
        );

        //specify the token struct that contains the owner ID
        let token = Token {
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //add timestamp
        if metadata.issued_at.is_none() {
            metadata.issued_at = Some(env::block_timestamp());
        }

        //verify user type
        if let Some(user_type) = metadata.user_type.clone() {
            let _ = UserType::from(user_type);
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    // TODO nft_modify_token
    #[payable]
    pub fn nft_modify_token(&mut self, token_id: TokenId, new_metadata: TokenMetadata) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        assert!(
            self.verify_admin_token(&env::predecessor_account_id()),
            "Only Admin can modify NFTs."
        );

        let mut token_metadata = self
            .token_metadata_by_id
            .get(&token_id)
            .unwrap_or_else(|| panic!("Token with ID: {} does not exist!", token_id.to_string()));

        if new_metadata.title.is_some() {
            token_metadata.title = new_metadata.title;
        }
        if new_metadata.description.is_some() {
            token_metadata.description = new_metadata.description;
        }
        if new_metadata.media.is_some() {
            token_metadata.media = new_metadata.media;
        }
        if new_metadata.media_hash.is_some() {
            token_metadata.media_hash = new_metadata.media_hash;
        }
        if new_metadata.copies.is_some() {
            token_metadata.copies = new_metadata.copies;
        }
        if new_metadata.issued_at.is_some() {
            token_metadata.issued_at = new_metadata.issued_at;
        }
        if new_metadata.expires_at.is_some() {
            token_metadata.expires_at = new_metadata.expires_at;
        }
        if new_metadata.starts_at.is_some() {
            token_metadata.starts_at = new_metadata.starts_at;
        }
        if new_metadata.extra.is_some() {
            token_metadata.extra = new_metadata.extra;
        }
        if new_metadata.reference.is_some() {
            token_metadata.reference = new_metadata.reference;
        }
        if new_metadata.reference_hash.is_some() {
            token_metadata.reference_hash = new_metadata.reference_hash;
        }

        if new_metadata.user_type.is_some() {
            let _ = UserType::from(new_metadata.user_type.clone().unwrap());
            token_metadata.user_type = new_metadata.user_type;
        }

        if new_metadata.organization.is_some() {
            token_metadata.organization = new_metadata.organization;
        }

        if new_metadata.updated_at.is_some() {
            token_metadata.updated_at = new_metadata.updated_at;
        } else {
            token_metadata.updated_at = Some(env::block_timestamp());
        }

        //insert the token ID and metadata
        self.token_metadata_by_id.insert(&token_id, &token_metadata);

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
