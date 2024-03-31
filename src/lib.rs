use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct NFTLeaf {
    pub nft_id: String,
    pub owner: AccountId,
    pub metadata: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CompressedNFTContract {
    merkle_tree: UnorderedMap<String, Vec<u8>>,
    authorized_account: AccountId,
}

#[near_bindgen]
impl CompressedNFTContract {
    #[init]
    pub fn new(authorized_account: AccountId) -> Self {
        Self {
            merkle_tree: UnorderedMap::new(b"m".to_vec()),
            authorized_account,
        }
    }

    pub fn update_merkle_tree(&mut self, leaf: NFTLeaf, proof: Vec<Vec<u8>>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.authorized_account,
            "Only authorized account can update the Merkle tree"
        );

        let leaf_hash = env::sha256(&borsh::to_vec(&leaf).unwrap());
        let mut current_hash = leaf_hash.to_vec();

        for p in proof {
            if current_hash <= p {
                current_hash = env::sha256(&[current_hash, p].concat()).to_vec();
            } else {
                current_hash = env::sha256(&[p, current_hash].concat()).to_vec();
            }
        }

        self.merkle_tree.insert(&leaf.nft_id, &current_hash);
    }

    pub fn transfer_nft(&mut self, nft_id: String, new_owner: AccountId, proof: Vec<Vec<u8>>) {
        let leaf = self.get_leaf(nft_id.clone()).expect("NFT not found");
        assert_eq!(
            env::predecessor_account_id(),
            leaf.owner,
            "Only the owner can transfer the NFT"
        );

        let mut current_hash = env::sha256(&borsh::to_vec(&leaf).unwrap());

        for p in proof {
            if current_hash <= p {
                current_hash = env::sha256(&[current_hash, p].concat()).to_vec();
            } else {
                current_hash = env::sha256(&[p, current_hash].concat()).to_vec();
            }
        }

        assert_eq!(
            current_hash,
            self.merkle_tree.get(&nft_id).unwrap(),
            "Invalid Merkle proof"
        );

        let new_leaf = NFTLeaf {
            nft_id: nft_id.clone(),
            owner: new_owner,
            metadata: leaf.metadata,
        };

        self.update_merkle_tree(new_leaf, proof);
    }

    pub fn get_leaf(&self, nft_id: String) -> Option<NFTLeaf> {
        self.merkle_tree.get(&nft_id).map(|hash| {
            let data = env::storage_read(&hash).unwrap();
            borsh::from_slice(&data).unwrap()
        })
    }
}
