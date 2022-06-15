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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let test_lookup_map = LookupMap::new("test");
        println!("Woohoo! We testin'!")
    }
}