/// lib.rs
/// Main "entrypoint" for the Breakfast Club NFT contract.
/// @author bradlet <bradlet2@pdx.edu>

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Account ID of contract owner
    pub owner_id: AccountId,
    // Map of all token ID's currently owned for a given account
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    // Map tracking all Tokens by their TokenId
    pub tokens_by_id: LookupMap<TokenId, Token>,
    // Map tracking all Tokens' metadata by TokenId
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    // Metadata for this contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

#[cfg(test)]
mod tests {

    #[test]
    fn first_test() {
        println!("Woohoo! We testin'!")
    }
}