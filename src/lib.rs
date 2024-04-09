#![allow(non_snake_case)]

use near_sdk::{near_bindgen, AccountId, env};
use near_sdk::require;
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::{Sha256, Digest};
use merkletree::hash::Algorithm;
use std::hash::Hasher;
use serde_json;
use merkletree::proof::Proof;
use typenum::U2;
type BaseTreeArity = U2;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use anyhow::Error;



impl Hasher for Sha256Algorithm {
    fn write(&mut self, _bytes: &[u8]) {
        // Implement this method to update the hasher with the provided bytes
    }

    fn finish(&self) -> u64 {
        // Convert the hash stored in `self.data` to a `u64`
        let mut result = 0;
        for &byte in &self.data[..8] {
            result = (result << 8) | (byte as u64);
        }
        result
    }
}

pub struct Sha256Algorithm {
    data: [u8; 32],
}

impl Algorithm<[u8; 32]> for Sha256Algorithm {
    fn hash(&mut self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.data);
        let result = hasher.finalize();
        result.into()
    }
}

impl Default for Sha256Algorithm {
    fn default() -> Self {
        Sha256Algorithm {
            data: [0; 32], // Default value for data
        }
    }
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
pub struct CompressedNFTContract {
    owner: AccountId,
    merkle_tree_data: Vec<[u8; 32]>,
    nft_owners: HashMap<String, AccountId>,
}

impl CompressedNFTContract {
    pub fn new(owner: AccountId, merkle_tree_data: Vec<[u8; 32]>) -> Self {
        Self {
            owner,
            merkle_tree_data,
            nft_owners: HashMap::new(),
        }
    }

    pub fn get_merkle_tree(&self) -> MerkleTree<[u8; 32], Sha256Algorithm, VecStore<[u8; 32]>> {
        MerkleTree::from_data(&self.merkle_tree_data).unwrap()
    }

    // Method to update the Merkle root, restricted to the contract owner
    pub fn update_merkle_root(&mut self, new_merkle_root: [u8; 32]) {
        require!(
            env::signer_account_id() == self.owner,
            "Only the owner can update the Merkle root"
        );
        self.merkle_tree_data = vec![new_merkle_root];
    }

    pub fn transfer_nft(&mut self, receiver_id: AccountId, nft_id: String, merkle_proof: Result<Proof<[u8; 32]>, Error>) {
        // Verify the Merkle proof
        require!(self.verify_merkle_proof(&nft_id, merkle_proof), "Invalid Merkle proof");
    
        // Update the NFT's ownership
        self.nft_owners.insert(nft_id, receiver_id);
    }

    pub fn update_merkle_root_after_mint(&mut self, new_merkle_root: [u8; 32]) {
        // In a real scenario, this method would be protected and called by an authorized account
        // after off-chain processing (e.g., the account that manages the off-chain indexer).
        require!(
            env::signer_account_id() == self.owner,
            "Only the owner can update the Merkle root"
        );
    
        // Append the new Merkle root to the Merkle tree data
        self.merkle_tree_data.push(new_merkle_root);
    
        // Would need to log the mint event
    }

    // Helper method to verify a Merkle proof
    fn verify_merkle_proof(&self, _nft_id: &str, proof: Result<Proof<[u8; 32], BaseTreeArity>, Error>) -> bool {
        match proof {
            Ok(proof) => {
                proof.validate::<Sha256Algorithm>().unwrap_or(false)
            },
            Err(_) => false,
        }
    }

    /// Utils to serialize and deserialize the Merkle tree data

    // Serialize the Merkle tree data
    pub fn serialize_merkle_tree_data(&self) -> Vec<u8> {
        let mut serialized_data = vec![0u8; 32 * self.merkle_tree_data.len()];
        let mut start = 0;
        let mut end = 32;
        for element in &self.merkle_tree_data {
            serialized_data[start..end].copy_from_slice(element);
            start += 32;
            end += 32;
        }
        serialized_data
    }

    pub fn deserialize_merkle_tree(&mut self, data: &[u8]) {
        // Deserialize the leaves of the MerkleTree
        let leaves: Vec<[u8; 32]> = serde_json::from_slice(data).unwrap();
    
        // Rebuild the MerkleTree from the leaves
        self.merkle_tree_data = leaves;
    }
}