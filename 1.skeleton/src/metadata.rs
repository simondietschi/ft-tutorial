use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{ext_contract, near};

use crate::*;

#[derive(Clone)]
#[near(serializers=[borsh, json])]
pub struct FungibleTokenMetadata {
    /*
        FILL THIS IN
    */
}

#[ext_contract(ext_ft_metadata)]
pub trait FungibleTokenMetadataProvider {
    // View call for returning the contract metadata
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}

#[near]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        /*
            FILL THIS IN
        */
        todo!(); //remove once code is filled in.
    }
}
