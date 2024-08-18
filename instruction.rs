use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

use crate::state::{NFTMetadata, GameTransaction};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GameInstruction {
    MintNFT {
        metadata: NFTMetadata,
    },
    TransferToken {
        amount: u64,
    },
    ExecuteGameTransaction {
        transaction: GameTransaction,
    },
}

impl GameInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        GameInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}