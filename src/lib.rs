use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


// Import other modules
mod instruction;
mod processor;
mod state;


// Declare and export the program's entrypoint
entrypoint!(process_instruction);


// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // Unpack the instruction data and process it
    instruction::GameInstruction::unpack(instruction_data)
        .and_then(|instruction| processor::process(program_id, accounts, instruction))
}


// Declare the program ID
solana_program::declare_id!("Your_Program_ID_Here");