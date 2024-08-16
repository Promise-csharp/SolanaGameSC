use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    sysvar::rent::Rent,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum GameInstruction {
    // Instruction to purchase an item
    PurchaseItem { item_id: u8 },
    // Instruction to use an item
    UseItem { item_id: u8 },
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GameItem {
    pub id: u8,
    pub owner: Pubkey,
    pub value: u64, // e.g., price or effect value
    pub is_in_use: bool,
}

pub struct GameState {
    pub items: Vec<GameItem>,
}

impl GameState {
    pub fn new() -> Self {
        GameState { items: Vec::new() }
    }

    pub fn add_item(&mut self, item: GameItem) {
        self.items.push(item);
    }

    pub fn get_item(&self, item_id: u8) -> Option<&GameItem> {
        self.items.iter().find(|item| item.id == item_id)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::try_from_slice(instruction_data)?;

    match instruction {
        GameInstruction::PurchaseItem { item_id } => {
            msg!("Processing Purchase Item Instruction");
            process_purchase_item(program_id, accounts, item_id)
        }
        GameInstruction::UseItem { item_id } => {
            msg!("Processing Use Item Instruction");
            process_use_item(program_id, accounts, item_id)
        }
    }
}

fn process_purchase_item(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    item_id: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer_account = next_account_info(account_info_iter)?;
    let item_account = next_account_info(account_info_iter)?;

    // Ensure the payer is a signer
    if !payer_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load item data
    let mut item_data = GameItem::try_from_slice(&item_account.data.borrow())?;
    
    // Check if the item is available for purchase
    if item_data.is_in_use {
        return Err(ProgramError::Custom(0)); // Item already in use
    }

    // Set the owner and mark the item as in use
    item_data.owner = *payer_account.key;
    item_data.is_in_use = true;

    // Save updated item data
    item_data.serialize(&mut &mut item_account.data.borrow_mut()[..])?;
    msg!("Item purchased successfully");

    Ok(())
}



fn process_use_item(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    item_id: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let player_account = next_account_info(account_info_iter)?;
    let item_account = next_account_info(account_info_iter)?;

    // Load item data
    let item_data = GameItem::try_from_slice(&item_account.data.borrow())?;

    if item_data.owner != *player_account.key {
        return Err(ProgramError::Custom(1)); // Not the owner of the item
    }

    // Apply item effect (e.g., increase health or gun damage)
    // (This is just a placeholder for the actual game logic)
    msg!("Item used: ID {}, Effect Value {}", item_id, item_data.value);

    Ok(())
}
