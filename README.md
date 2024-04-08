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


## TODO:
- proofs
- mint through single minter (ShardDog)
- transfer logic


