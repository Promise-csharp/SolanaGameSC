use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;


// Define the metadata structure for NFTs
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NFTMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}


// Define the structure for game transactions
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameTransaction {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}
