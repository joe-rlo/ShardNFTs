# ShardNFTs

cargo-near-new-project-description

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Iteract with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)

NOTES:
you can create a standard for compressed NFTs on NEAR:
 - publish a merkle root into smart contract
 - when transferring - need to post the merkle path with data, it validates it and updates the state root
 - have off-chain indexer that maintains the actual state and can be used to construct these merkle paths

I think this can be though even extension of existing NFT standard because you can add extra fields to NEAR methods but need wallets to get added

# CompressedNFTContract

The `CompressedNFTContract` is a smart contract written in Rust for the NEAR blockchain. It implements a Merkle tree data structure to efficiently store and manage non-fungible tokens (NFTs). The contract allows an authorized account to update the Merkle tree and enables everyone to read from the Merkle tree.

## Features

- Efficient storage of NFTs using a Merkle tree
- Authorized account for updating the Merkle tree
- Public access to read NFT data from the Merkle tree
- Transfer of NFTs using Merkle proofs

## Contract Structure

The contract consists of the following main components:

- `NFTLeaf`: A struct representing a leaf in the Merkle tree, containing the NFT ID, owner, and metadata.
- `CompressedNFTContract`: The main contract struct that holds the Merkle tree and the authorized account.

## Functions

### Initialization

- `new(authorized_account: AccountId) -> Self`: Initializes the contract with the specified authorized account.

### Merkle Tree Operations

- `update_merkle_tree(leaf: NFTLeaf, proof: Vec<Vec<u8>>)`: Allows the authorized account to update the Merkle tree by providing a new leaf and the corresponding Merkle proof.
- `get_leaf(nft_id: String) -> Option<NFTLeaf>`: Retrieves a leaf from the Merkle tree by providing the NFT ID. Returns an `Option<NFTLeaf>`, which will be `None` if the leaf doesn't exist.

### NFT Transfer

- `transfer_nft(nft_id: String, new_owner: AccountId, proof: Vec<Vec<u8>>)`: Allows the owner of an NFT to transfer it to a new owner using a Merkle proof. The function verifies the proof and updates the Merkle tree with the new owner.

## Usage

1. Deploy the `CompressedNFTContract` to the NEAR blockchain using the `new` function, specifying the authorized account that can update the Merkle tree.

2. The authorized account can update the Merkle tree by calling the `update_merkle_tree` function, providing a new leaf and the corresponding Merkle proof.

3. Anyone can retrieve the data of an NFT by calling the `get_leaf` function and providing the NFT ID.

4. The owner of an NFT can transfer it to a new owner by calling the `transfer_nft` function, providing the NFT ID, the new owner's account ID, and the Merkle proof.

## Security Considerations

- Only the authorized account can update the Merkle tree. Ensure that the authorized account is properly secured and managed.
- The contract assumes that the provided Merkle proofs are valid. Proper verification and error handling should be implemented to prevent invalid proofs from being accepted.
- Additional security measures, such as access control and input validation, should be implemented based on the specific requirements of the application.

## License

This contract is released under the [MIT License](https://opensource.org/licenses/MIT).