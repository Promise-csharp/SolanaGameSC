use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;


use crate::state::{NFTMetadata, GameTransaction};


// Define the instructions that the program can execute
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
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        GameInstruction::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}