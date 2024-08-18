use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NFTMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameTransaction {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}