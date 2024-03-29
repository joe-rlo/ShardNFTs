use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};
use merkle::MerkleTree; //Not sure if this is the right library

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CompressedNFTContract {
    owner: AccountId,
    merkle_root: Vec<u8>, 
}

#[near_bindgen]
impl CompressedNFTContract {
    // Initialize the contract with the owner account and an initial Merkle root
    #[init]
    pub fn new(owner: AccountId, merkle_root: Vec<u8>) -> Self {
        assert!(!merkle_root.is_empty(), "Merkle root cannot be empty");
        Self { owner, merkle_root }
    }

    // Method to update the Merkle root, restricted to the contract owner
    pub fn update_merkle_root(&mut self, new_merkle_root: Vec<u8>) {
        assert_eq!(
            near_sdk::env::signer_account_id(),
            self.owner,
            "Only the owner can update the Merkle root"
        );
        self.merkle_root = new_merkle_root;
    }

    // Transfer method requiring a Merkle proof
    pub fn transfer_nft(&mut self, receiver_id: AccountId, nft_id: String, merkle_proof: Vec<Vec<u8>>) {
        // Logic to update the NFT's ownership state.
       
    }

    pub fn update_merkle_root_after_mint(&mut self, new_merkle_root: Vec<u8>) {
        // In a real scenario, this method would be protected and called by an authorized account
        // after off-chain processing (e.g., the account that manages the off-chain indexer).
        assert_eq!(
            env::signer_account_id(),
            self.owner,
            "Only the owner can update the Merkle root"
        );
        self.merkle_root = new_merkle_root;
        // Would need to log the mint event
    }
}

// TODOS: logic for Merkle proof verification, NFT state management, ... ????
