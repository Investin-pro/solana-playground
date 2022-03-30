#![cfg(not(feature = "no-entrypoint"))]

use solana_program::{pubkey::Pubkey, account_info::AccountInfo, entrypoint::{ProgramResult}, entrypoint};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)?;
    Ok(())
}