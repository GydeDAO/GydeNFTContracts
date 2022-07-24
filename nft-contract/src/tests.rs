/* unit tests */
#[cfg(test)]
use crate::Contract;
use crate::{TokenMetadata, UserType};
use near_sdk::json_types::U128;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::testing_env;
use near_sdk::{env, AccountId};

const MINT_STORAGE_COST: u128 = 100_000_000_000_000_000_000_000;

fn get_context(predecessor: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder.predecessor_account_id(predecessor);
    builder
}

fn sample_token_metadata() -> TokenMetadata {
    TokenMetadata {
        title: Some("Gyde Test".into()),
        description: Some("This is a smart contract test".into()),
        media: None,
        media_hash: None,
        copies: Some(1u64),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
        user_type: None,
        organization: None,
    }
}

fn gyde_token_metadata_sample(
    user_type: Option<String>,
    organization: Option<String>,
) -> TokenMetadata {
    TokenMetadata {
        title: Some("Gyde Test".into()),
        description: Some("This is a smart contract test".into()),
        media: None,
        media_hash: None,
        copies: Some(1u64),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
        user_type: user_type,
        organization: organization,
    }
}

#[test]
#[should_panic(expected = "The contract is not initialized")]
fn test_default() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let _contract = Contract::default();
}

#[test]
fn test_new_account_contract() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into());
    testing_env!(context.is_view(true).build());
    let contract_nft_tokens = contract.nft_tokens(Some(U128(0)), None);
    assert_eq!(contract_nft_tokens.len(), 0);
}

#[test]
fn test_mint_nft() {
    let mut context = get_context(accounts(0));

    testing_env!(context.build());

    let mut contract = Contract::new_default_meta(accounts(0));
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());

    let token_metadata: TokenMetadata = gyde_token_metadata_sample(Some("admin".to_string()), None);
    let token_id = "0".to_string();
    contract.nft_mint(token_id.clone(), token_metadata, accounts(0));
    let contract_nft_tokens = contract.nft_tokens(Some(U128(0)), None);
    assert_eq!(contract_nft_tokens.len(), 1);

    assert_eq!(contract_nft_tokens[0].token_id, token_id);
    assert_eq!(contract_nft_tokens[0].owner_id, accounts(0));
    assert_eq!(
        contract_nft_tokens[0].metadata.title,
        sample_token_metadata().title
    );
    assert_eq!(
        contract_nft_tokens[0].metadata.description,
        sample_token_metadata().description
    );

    assert_eq!(
        contract_nft_tokens[0].metadata.media,
        sample_token_metadata().media
    );

    assert_eq!(
        contract_nft_tokens[0].metadata.user_type,
        Some(UserType::Admin.to_string())
    );
}

#[test]
#[should_panic(expected = "Caller is not an Admin.")]
fn test_mint_nft_as_user() {
    let mut context = get_context(accounts(0));

    testing_env!(context.build());

    let mut contract = Contract::new_default_meta(accounts(0));
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(1))
        .build());

    let token_metadata: TokenMetadata = gyde_token_metadata_sample(Some("user".to_string()), None);
    let token_id = "0".to_string();

    contract.nft_mint(token_id.clone(), token_metadata, accounts(0));
}

#[test]
fn test_mint_nft_as_admin() {
    // Steps:
    // 1. Mint an NFT admin to another account
    // 2. Mint an NFT using the other account and see it passes

    let mut context = get_context(accounts(0));

    testing_env!(context.build());

    let mut contract = Contract::new_default_meta(accounts(0).into());
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());

    let token_metadata: TokenMetadata = gyde_token_metadata_sample(Some("admin".to_string()), None);
    let token_id = "0".to_string();

    contract.nft_mint(token_id.clone(), token_metadata, accounts(1));
    let supply = contract.nft_supply_for_owner(accounts(1).into());
    assert!(supply == near_sdk::json_types::U128(1));

    // Mint NFT from account 1
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(1))
        .build());

    let token_id = "1".to_string();
    contract.nft_mint(
        token_id.clone(),
        gyde_token_metadata_sample(Some("super_user".to_string()), None),
        accounts(1),
    );
    let supply = contract.nft_supply_for_owner(accounts(1).into());
    assert!(supply == near_sdk::json_types::U128(2));
}

#[test]
#[should_panic(expected = "Tokens can be transfered by Admins only.")]
fn test_internal_transfer_by_user() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("user")), None),
        accounts(1),
    );

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(1))
        .build());
    contract.internal_transfer(&accounts(1), &accounts(0), &token_id.clone(), None);
}
#[test]
fn test_internal_transfer() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0));

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(1)
        .predecessor_account_id(accounts(0))
        .build());
    contract.internal_transfer(&accounts(0), &accounts(1), &token_id.clone(), None);

    testing_env!(context
        .storage_usage(env::storage_usage())
        .account_balance(env::account_balance())
        .is_view(true)
        .attached_deposit(0)
        .build());

    let tokens = contract.nft_tokens_for_owner(accounts(1), Some(U128(0)), None);
    assert_ne!(
        tokens.len(),
        0,
        "Token not correctly created and/or sent to second account"
    );
    let token = &tokens[0];
    assert_eq!(token.token_id, token_id);
    assert_eq!(token.owner_id, accounts(1));
    assert_eq!(token.metadata.title, sample_token_metadata().title);
    assert_eq!(
        token.metadata.description,
        sample_token_metadata().description
    );
    assert_eq!(token.metadata.media, sample_token_metadata().media);
}

#[test]
fn test_internal_remove_token_from_owner() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0));

    let contract_nft_tokens_before = contract.nft_tokens_for_owner(accounts(0), None, None);
    assert_eq!(contract_nft_tokens_before.len(), 1);

    contract.internal_remove_token_from_owner(&accounts(0), &token_id);
    let contract_nft_tokens_after = contract.nft_tokens_for_owner(accounts(0), None, None);
    assert_eq!(contract_nft_tokens_after.len(), 0);
}

#[test]
fn test_nft_total_supply() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(token_id.clone(), sample_token_metadata(), accounts(0));

    let total_supply = contract.nft_total_supply();
    assert_eq!(total_supply, U128(1));
}

#[test]
fn test_nft_modify_admin() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("user")), None),
        accounts(1),
    );
    let old_tokens = contract.nft_tokens_for_owner(accounts(1), Some(U128(0)), None);
    assert_eq!(old_tokens.len(), 1);

    let old_token = old_tokens.get(0).unwrap();
    assert_eq!(old_token.metadata.user_type.clone().unwrap(), UserType::User.to_string());

    contract.nft_modify_token(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("admin")), Some("gyde_test".to_string())),
    );

    let new_tokens = contract.nft_tokens_for_owner(accounts(1), Some(U128(0)), None);
    assert_eq!(new_tokens.len(), 1);

    let new_token = new_tokens.get(0).unwrap();
    assert_eq!(new_token.metadata.user_type.clone().unwrap(), UserType::Admin.to_string());
                 
}

#[test]
#[should_panic(expected = "Only Admin can modify NFTs.")]
fn test_nft_modify_as_user() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("user")), None),
        accounts(1),
    );
    let old_tokens = contract.nft_tokens_for_owner(accounts(1), Some(U128(0)), None);
    assert_eq!(old_tokens.len(), 1);

    let old_token = old_tokens.get(0).unwrap();
    assert_eq!(old_token.metadata.user_type.clone().unwrap(), UserType::User.to_string());
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(1))
        .build());

    contract.nft_modify_token(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("admin")), Some("gyde_test".to_string())),
    );

    let new_tokens = contract.nft_tokens_for_owner(accounts(1), Some(U128(0)), None);
    assert_eq!(new_tokens.len(), 1);

    let new_token = new_tokens.get(0).unwrap();
    assert_eq!(new_token.metadata.user_type.clone().unwrap(), UserType::Admin.to_string());
}

#[test]
#[should_panic(expected = "Invalid user type!")]
pub fn test_nft_mint_invalid_user() {
    let mut context = get_context(accounts(0));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(0).into());

    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(MINT_STORAGE_COST)
        .predecessor_account_id(accounts(0))
        .build());
    let token_id = "0".to_string();
    contract.nft_mint(
        token_id.clone(),
        gyde_token_metadata_sample(Some(String::from("no_user")), None),
        accounts(1),
    );
}