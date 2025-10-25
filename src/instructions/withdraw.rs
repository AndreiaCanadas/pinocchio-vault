use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, msg, program_error::ProgramError, pubkey, sysvars::{rent::Rent, Sysvar}, ProgramResult
};
use pinocchio_system::instructions::Transfer;

use crate::state::Vault;

/// # Withdraw Instruction
/// 
/// This function allows a user to withdraw SOL from the existing vault PDA.
/// It closes the accounts if the amount remaining in the vault is less than the rent-exempt amount.
/// 
/// ## Business Logic:
/// 1. User withdraws SOL from the vault
/// 
/// ## Accounts expected:
/// 0. [signer] user - The user who is withdrawing SOL
/// 1. [writable] vault - The vault PDA that holds the SOL
/// 2. [] vault_state - The vault state PDA that holds the vault bump and state bump
/// 3. [] system_program - System program
/// 
/// ## Data parameters:
/// 0. [u8; 8] amount - The amount of SOL to withdraw
pub fn process_withdraw_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    msg!("Processing withdraw instruction");

    // Unpack accounts - Validate expected accounts
    let [user, vault, vault_state, _system_program, _remaining @.. ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Check if user is the signer
    if !user.is_signer() {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Check if vault is owned by the system program
    if !vault.is_owned_by(&pinocchio_system::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Check if vault state is owned by the program
    if !vault_state.is_owned_by(&crate::ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Access vault state account
    let vault_state_account = Vault::from_account_info(vault_state)?;
    let vault_bump = vault_state_account.vault_bump;
    let state_bump = vault_state_account.state_bump;

    // Validate vault state PDA
    let state_seed = [(b"state"), user.key().as_slice(), &state_bump];
    let state_seeds = &state_seed[..];
    let state_pda = pubkey::checked_create_program_address(state_seeds, &crate::ID).unwrap();
    assert_eq!(&state_pda, vault_state.key());

    // Validate vault PDA
    let vault_seed = [(b"vault"), state_pda.as_ref(), &vault_bump];
    let vault_seeds = &vault_seed[..];
    let vault_pda = pubkey::checked_create_program_address(vault_seeds, &crate::ID).unwrap();
    // let vault_pda = pinocchio_pubkey::derive_address(&vault_seeds, None, &crate::ID);
    // pubkey::find_program_address
    assert_eq!(&vault_pda, vault.key());

    // Prepare seeds for signing
    let signer_seed = [Seed::from(b"vault"), Seed::from(state_pda.as_ref()), Seed::from(&vault_bump)];
    let signers = Signer::from(&signer_seed);

    // Validate Data Parameters
    if data.len() != 8 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Unpack data
    let amount = u64::from_le_bytes(data[..8].try_into().unwrap());

    // Check that amount in vault is greater than amount to withdraw
    if amount > vault.lamports() {
        return Err(ProgramError::InvalidInstructionData);
    }
    // Check that amount is greater than 0
    else if amount == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }
    // Check if amount left in vault is greater than rent-exempt amount
    else if vault.lamports() - amount < Rent::get()?.minimum_balance(0) {
        
        // Withdraw all SOL from vault
        Transfer {
            from: vault,
            to: user,
            lamports: vault.lamports(),
        }.invoke_signed(&[signers])?;

        // Manually close vault state account and return rent to user
        unsafe {
            *user.borrow_mut_lamports_unchecked() += *vault_state.borrow_lamports_unchecked();
            *vault_state.borrow_mut_lamports_unchecked() = 0;
        };

        msg!("SOL balance withdrawn from vault and closed accounts");
    }
    else{
        // Withdraw amount
        Transfer {
            from: vault,
            to: user,
            lamports: amount,
        }.invoke_signed(&[signers])?;

        msg!("SOL amount withdrawn from vault");
    }

    Ok(())
}