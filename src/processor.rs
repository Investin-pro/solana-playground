use crate::error::SwapError;
use crate::instruction::SwapInstruction;
use arrayref::array_ref;
use bytemuck::bytes_of;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::state::Account;

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // always have to get an accounts iter to loop thru the accounts;
    let accounts_iter = &mut accounts.iter();

    let instruction = SwapInstruction::unpack(instruction_data)?;

    match instruction {
        SwapInstruction::Initialize { amount } => {
            msg!("Initialized");
        }
        SwapInstruction::Swap { amount } => {
            msg!("Swapping {}", amount)
        }
    }

    Ok(())
}
