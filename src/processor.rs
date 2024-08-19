use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    program::{invoke, invoke_signed},
};
use spl_token::instruction as token_instruction;

use crate::instruction::GameInstruction;
use crate::state::{NFTMetadata, GameTransaction};

// Main processing function that routes to the appropriate instruction handler
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction: GameInstruction) -> ProgramResult {
    match instruction {
        GameInstruction::MintNFT { metadata } => {
            mint_nft(program_id, accounts, metadata)
        }
        GameInstruction::TransferToken { amount } => {
            transfer_token(accounts, amount)
        }
        GameInstruction::ExecuteGameTransaction { transaction } => {
            execute_game_transaction(accounts, transaction)
        }
    }
}

// Function to mint a new NFT
fn mint_nft(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    metadata: NFTMetadata,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get account information
    let mint_account = next_account_info(account_info_iter)?;
    let mint_authority = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let rent_sysvar = next_account_info(account_info_iter)?;

    // Calculate rent
    let rent = Rent::from_account_info(rent_sysvar)?;
    let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

    // Create mint account
    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            mint_rent,
            spl_token::state::Mint::LEN as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_account.clone(), system_program.clone()],
    )?;

    // Initialize mint
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            mint_authority.key,
            Some(mint_authority.key),
            0,
        )?,
        &[mint_account.clone(), rent_sysvar.clone(), token_program.clone()],
    )?;

    // Log NFT information
    msg!("NFT minted successfully");
    msg!("Name: {}", metadata.name);
    msg!("Symbol: {}", metadata.symbol);
    msg!("URI: {}", metadata.uri);

    Ok(())
}

// Function to transfer tokens
fn transfer_token(
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get account information
    let source = next_account_info(account_info_iter)?;
    let destination = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // Execute token transfer
    invoke(
        &token_instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source.clone(), destination.clone(), authority.clone(), token_program.clone()],
    )?;


    // Log transfer information
    msg!("Token transfer successful");
    msg!("Amount: {}", amount);

    Ok(())

}

// Function to execute in-game transactions
fn execute_game_transaction(
    accounts: &[AccountInfo],
    transaction: GameTransaction,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    
    // Get account information
    let from_account = next_account_info(account_info_iter)?;
    let to_account = next_account_info(account_info_iter)?;
    let authority = next_account_info(account_info_iter)?;

    // Verify transaction details
    if from_account.key != &transaction.from || to_account.key != &transaction.to {
        return Err(ProgramError::InvalidAccountData.into());
    }

    // Check if the authority is the signer
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature.into());
    }

    // Log transaction information
    msg!("Executing game transaction");
    msg!("From: {}", transaction.from);
    msg!("To: {}", transaction.to);
    msg!("Amount: {}", transaction.amount);

    // TODO: Implement actual token transfer logic here

    Ok(())
}
