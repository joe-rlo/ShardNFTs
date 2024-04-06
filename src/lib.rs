use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, PanicOnDefault};
use merkle::MerkleTree; //Not sure if this is the right library

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CompressedNFTContract {
    owner: AccountId,
    merkle_root: Vec<u8>, 
    authorized_accounts: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl CompressedNFTContract {
    // Initialize the contract with the owner account and an initial Merkle root
    #[init]
    pub fn new(owner: AccountId, merkle_root: Vec<u8>) -> Self {
        assert!(!merkle_root.is_empty(), "Merkle root cannot be empty");
        let mut authorized_accounts = UnorderedSet::new(b"authorized".to_vec());
        authorized_accounts.insert(&owner); // Automatically authorize the owner
        Self { owner, merkle_root, authorized_accounts }
    }

    // Method to update the Merkle root, restricted to the contract owner
    pub fn update_merkle_root(&mut self, new_merkle_root: Vec<u8>) {
        let signer = near_sdk::env::signer_account_id();
        assert!(
            self.authorized_accounts.contains(&signer) || signer == self.owner,
            "Only the owner or an authorized account can update the Merkle root"
        );
        self.merkle_root = new_merkle_root;
        env::log_str(&format!("Merkle root updated to: {:?}", new_merkle_root));
    }

    // Transfer method requiring a Merkle proof
    pub fn transfer_nft(&mut self, receiver_id: AccountId, nft_id: String, merkle_proof: Vec<Vec<u8>>) {
        // Logic to update the NFT's ownership state.
       
    }

     // Method to add an authorized account
     pub fn add_authorized_account(&mut self, account_id: AccountId) {
        assert_eq!(
            env::signer_account_id(),
            self.owner,
            "Only the owner can add authorized accounts"
        );
        self.authorized_accounts.insert(&account_id);
    }

    // Method to remove an authorized account
    pub fn remove_authorized_account(&mut self, account_id: AccountId) {
        assert_eq!(
            env::signer_account_id(),
            self.owner,
            "Only the owner can remove authorized accounts"
        );
        self.authorized_accounts.remove(&account_id);
    }

    pub fn update_merkle_root_after_mint(&mut self, new_merkle_root: Vec<u8>) {
        let signer = env::signer_account_id();
        assert!(
            self.authorized_accounts.contains(&signer) || signer == self.owner,
            "Unauthorized account"
        );
        self.merkle_root = new_merkle_root;
        
        env::log_str(&format!("Merkle root updated to: {:?}", new_merkle_root));

        /*Maybe still call the normal mint log
         let event_data = json!({
            "standard": "nep171",
            "version":"1.1.0",
            "event": "nft_mint",
            "data": [
                {
                    "owner_id": receiver_id,
                    "token_ids": [formatted_string],
                }
            ]
        });
        env::log_str(&format!("EVENT_JSON:{}", event_data.to_string()));
        */
    }
}

// TODOS: logic for Merkle proof verification, NFT state management, ... ????
