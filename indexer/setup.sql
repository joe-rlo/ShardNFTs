CREATE TABLE nfts (
    nft_id UUID PRIMARY KEY,
    owner VARCHAR(255) NOT NULL,
    metadata TEXT NOT NULL
);

# Just as a note, this table is not needed since we are using IPFS for metadata storage. But it is here for reference.

CREATE TABLE nft_metadata (
    nft_id UUID PRIMARY KEY,
    title VARCHAR(255),
    description TEXT,
    media VARCHAR(255),
    animation_url VARCHAR(255),
    reference VARCHAR(255),
    cid VARCHAR(255),
    FOREIGN KEY (nft_id) REFERENCES nfts(nft_id) ON DELETE CASCADE
);

