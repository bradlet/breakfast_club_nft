/// lib.rs
/// Main "entrypoint" for the Breakfast Club NFT contract.
/// @author bradlet <bradlet2@pdx.edu>

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, BorshStorageKey};
use near_sdk::collections::{LazyOption};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token_manager: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval,
}

#[near_bindgen]
impl Contract {

    /// Initialize the contract; can only be called once.
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Contract has already been initialized.");
        metadata.assert_valid();
        Self {
            token_manager: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval)
            ),
            metadata: LazyOption::new(StorageKey::Metadata, Some(&metadata))
        }
    }

    /// Mint a new token of id `token_id` for new token owner `receiver_id`
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_meta: TokenMetadata
    ) -> Token {
        self.token_manager
            .internal_mint(token_id, receiver_id, Some(token_meta))
    }

}

near_contract_standards::impl_non_fungible_token_core!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_approval!(Contract, tokens);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, tokens);


// Note: Just copied base tests from tutorial so I can have an easier time playing around
// with the contract in an IDE.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_contract_standards::non_fungible_token::metadata::NFT_METADATA_SPEC;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;
    use near_sdk::MockedBlockchain;
    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    fn sample_contract_metadata() -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Breakfast Club NFT".to_string(),
            symbol: "BKFST".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None
        }
    }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Breakfast Sandwich with Egg".into()),
            description: Some("A delicious meal".into()),
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
        }
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Contract::new(
            accounts(1).into(),
            sample_contract_metadata()
        );
        testing_env!(context.is_view(true).build());
        assert_eq!(contract.nft_token("1".to_string()), None);
    }
}