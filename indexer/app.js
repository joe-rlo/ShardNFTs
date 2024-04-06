const express = require('express');
const { connect, KeyPair, keyStores, utils, WalletConnection, Contract} = require("near-api-js");
const bodyParser = require('body-parser');
const { MerkleTree } = require('merkletreejs');
const keccak256 = require('keccak256');
const { v4 } = require('uuid');

// Configuration and Initialization
const app = express();
const port = 3000;
app.use(bodyParser.json());
const db = require('./db');



async function getContract(accountId) {
    const myKeyStore = new keyStores.InMemoryKeyStore();
const PRIVATE_KEY = "ed25519:xxxx";
// creates a public / private key pair using the provided private key
const keyPair = KeyPair.fromString(PRIVATE_KEY);
// adds the keyPair you created to keyStore
await myKeyStore.setKey("testnet", ".testnet", keyPair);


// NEAR Configuration - Adjust as needed
const NEAR_CONFIG = {
    networkId: "testnet",
    contractName: "_______.testnet",
    keyStore: myKeyStore,
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
};

    const near = await connect(NEAR_CONFIG);

    const account = await near.account("_______.testnet");
    
    return new Contract(account, NEAR_CONFIG.contractName, {
        viewMethods: ['get_leaf'],
        changeMethods: ['update_merkle_tree', 'transfer_nft'],
    });
}


async function addNFTToDatabase(nft) {
  const { nft_id, owner, metadata } = nft;
  console.log(nft_id, owner, metadata);
  const res = await db.query(
    'INSERT INTO nfts (nft_id, owner, metadata) VALUES ($1, $2, $3) RETURNING *',
    [nft_id, owner, metadata]
  );
  return res.rows[0];
}


async function getAllNFTData() {
    const res = await db.query('SELECT nft_id FROM nfts');
    return res.rows.map(row => row.nft_id);
}
  

async function recalculateMerkleTree(newNftId) {
    const nfts = await getAllNFTData();
    nfts.push(newNftId);
    const leaves = nfts.map(id => keccak256(id));
    const tree = new MerkleTree(leaves, keccak256, { sortPairs: true });
    return tree.getRoot().toString('hex');
}

//not sure if these next two are needed but just in-case
async function addNFTMetadata(nftId, metadata) {
    const { title, description, media, animation_url, reference, cid } = metadata;
    const res = await db.query(
      `INSERT INTO nft_metadata (nft_id, title, description, media, animation_url, reference, cid)
       VALUES ($1, $2, $3, $4, $5, $6, $7)
       RETURNING *`,
      [nftId, title, description, media, animation_url, reference, cid]
    );
    return res.rows[0];
  }

async function getNFTMetadata(nftId) {
    const res = await db.query(
      'SELECT title, description, media, animation_url, reference, cid FROM nft_metadata WHERE  = $1',
      [nftId]
    );
    return res.rows[0]; // Assuming nft_id is unique, there should only be one row.
}

// Function to update the Merkle tree on-chain
async function updateMerkleRootOnChain(accountId, leafData, proofData) {
    const contract = await getContract(accountId);

    // Call the smart contract method
    try {
        await contract.update_merkle_tree({
            leaf: leafData, 
            proof: proofData,
        }, 300000000000000, //Gas
        );
        console.log('Merkle root updated successfully');
    } catch (error) {
        console.error('Error updating Merkle root:', error);
        throw error;
    }
}

  // API Endpoints
  app.post('/mint-nft', async (req, res) => {
    const { metadataCid, ownerAccountId } = req.body; 
    try {
        const nftId = v4(); 
        console.log(nftId);
        const leafData = { nft_id: nftId, owner: ownerAccountId, metadata: metadataCid };
        console.log(leafData);
        await addNFTToDatabase(leafData);

        const newMerkleRoot = await recalculateMerkleTree(nftId);
        const proof = []; // Need proof generation logic

        // Update Merkle Root on-chain
        await updateMerkleRootOnChain(ownerAccountId, leafData, proof); 

        res.json({ success: true, nftId, newMerkleRoot });
    } catch (error) {
        console.error(error); 
        res.status(500).json({ success: false, error: error.message });
    }
});


app.post('/transfer-nft', async (req, res) => {
    const { accountId, nftId, newOwner, proof } = req.body;
    try {
        const contract = await getContract(accountId);
        await contract.transfer_nft({ nft_id: nftId, new_owner: newOwner, proof:[] });
        res.json({ success: true, message: 'NFT transferred successfully' });
    } catch (error) {
        res.status(500).json({ success: false, error: error.message });
    }
});

app.get('/nft/:nftId', async (req, res) => {
    const { accountId } = req.query;
    const { nftId } = req.params;
    try {
        const contract = await getContract(accountId);
        const leaf = await contract.get_leaf({ nft_id: nftId });
        //need to get metadata from ipfs and send it back
        const metadata = await getNFTMetadata(nftId);
        res.json({ success: true, leaf, metadata });
    } catch (error) {
        res.status(404).json({ success: false, error: 'NFT not found or error retrieving data' });
    }
});


app.listen(port, () => {
    console.log(`Server listening at http://localhost:${port}`);
});
