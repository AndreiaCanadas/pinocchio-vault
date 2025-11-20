// COMMENTED OUT FOR TESTING WITH LITESVM
//#![no_std]
// use pinocchio::nostd_panic_handler;

use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, program_error::ProgramError, ProgramResult};

entrypoint!(process_instruction);
// nostd_panic_handler!();

//pinocchio_pubkey::declare_id!("4ibrEMW5F6hKnkW4jVedswYv6H6VtwPN6ar6dvXDN1nT");

const ID: Pubkey = five8_const::decode_32_const("77777777777777777777777777777777777777777777");


mod state;
mod instructions;
use instructions::*;
mod tests;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    assert_eq!(program_id, &ID);

    let (discriminator, data) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstructions::try_from(*discriminator)? {
        VaultInstructions::Deposit => instructions::process_deposit_instruction(accounts, data),
        VaultInstructions::Withdraw => instructions::process_withdraw_instruction(accounts, data),
    }?;

    Ok(())
}