use near_sdk::{near_bindgen, AccountId, env};
use near_sdk::collections::UnorderedMap;
use near_sdk::require;
use merkletree::merkle::MerkleTree;
use merkletree::store::VecStore;
use sha2::{Sha256, Digest};
use merkletree::hash::Algorithm;
use std::hash::Hasher;
use serde::{Serialize, Deserialize};

impl Hasher for Sha256Algorithm {
    fn write(&mut self, bytes: &[u8]) {
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

#[derive(Serialize, Deserialize)]
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

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    result.into()
}

#[near_bindgen]
#[derive(Serialize, Deserialize)]
pub struct CompressedNFTContract {
    owner: AccountId,
    merkle_tree: MerkleTree<[u8; 32], Sha256Algorithm, VecStore<[u8; 32]>>,
    nft_owners: UnorderedMap<String, AccountId>,
}

#[near_bindgen]
impl CompressedNFTContract {
    #[init]
    pub fn new(owner: AccountId, merkle_root: [u8; 32]) -> Self {
        require!(!merkle_root.is_empty(), "Merkle root cannot be empty");
        let merkle_tree = MerkleTree::<[u8; 32], [u8; 32], VecStore<[u8; 32]>>::new(vec![merkle_root]);
        Self {
            owner,
            merkle_tree,
            nft_owners: UnorderedMap::new(b"n".to_vec()),
        }
    }

    // Method to update the Merkle root, restricted to the contract owner
    pub fn update_merkle_root(&mut self, new_merkle_root: [u8; 32]) {
        require!(
            env::signer_account_id() == self.owner,
            "Only the owner can update the Merkle root"
        );
        self.merkle_tree = MerkleTree::<[u8; 32], [u8; 32], VecStore<[u8; 32]>>::new(vec![new_merkle_root]);
    }

    // Transfer method requiring a Merkle proof
    pub fn transfer_nft(&mut self, receiver_id: AccountId, nft_id: String, merkle_proof: Vec<(u64, [u8; 32])>) {
        // Verify the Merkle proof
        require!(self.verify_merkle_proof(&nft_id, &merkle_proof), "Invalid Merkle proof");

        // Update the NFT's ownership
        self.nft_owners.insert(&nft_id, &receiver_id);
    }

    pub fn update_merkle_root_after_mint(&mut self, new_merkle_root: [u8; 32]) {
        // In a real scenario, this method would be protected and called by an authorized account
        // after off-chain processing (e.g., the account that manages the off-chain indexer).
        require!(
            env::signer_account_id() == self.owner,
            "Only the owner can update the Merkle root"
        );
        self.merkle_tree = MerkleTree::<[u8; 32], [u8; 32], VecStore<[u8; 32]>>::new(vec![new_merkle_root]);
        // Would need to log the mint event
    }

    // Helper method to verify a Merkle proof
    fn verify_merkle_proof(&self, nft_id: &str, proof: &[(u64, [u8; 32])]) -> bool {
        let leaf = sha256_hash(nft_id.as_bytes());
        self.merkle_tree.verify(proof, &leaf).unwrap_or(false)
    }
}