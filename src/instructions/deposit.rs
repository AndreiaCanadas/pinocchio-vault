use pinocchio::{
    account_info::AccountInfo, instruction::{Seed, Signer}, msg, program_error::ProgramError, pubkey::{self, log}, sysvars::{rent::Rent, Sysvar}, ProgramResult
};
use pinocchio_system::instructions::{CreateAccount, Transfer};

use crate::state::Vault;

/// # Deposit Instruction
/// 
/// This function allows a user to deposit SOL into a vault PDA (system account derived from the vault state PDA address).
/// It initializes the vault state PDA if it doesn't exist (Vault account derived from the user's public key).
/// This assumes that the vault PDA (system account) also needs to be initialized when the vault state PDA is initialized.
/// 
/// ## Business Logic:
/// 1. User sends SOL to the vault
/// 
/// ## Accounts expected:
/// 0. [signer] user - The user who is depositing SOL
/// 1. [writable] vault - The vault PDA (system account derived from the vault state PDA address) to hold the SOL
/// 2. [writable] vault_state - The vault state PDA (Vault account derived from the user's public key) to save the vault bump and state bump
/// 3. [] system_program - The system program for account creation
/// 
/// ## Data parameters:
/// 0. [u8; 1] vault_bump - The bump of the vault PDA
/// 1. [u8; 1] state_bump - The bump of the vault state PDA
/// 2. [u8; 8] amount - The amount of SOL to deposit
pub fn process_deposit_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {

    msg!("Processing deposit instruction");

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

    // Validate Data Parameters
    if data.len() != 10 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    // Unpack data
    let vault_bump = unsafe{ *(data.as_ptr() as *const u8) }.to_le_bytes();
    let state_bump = unsafe{ *(data.as_ptr().add(1) as *const u8) }.to_le_bytes();
    let amount = u64::from_le_bytes(data[2..10].try_into().unwrap());

    // Validate vault state PDA
    let state_seed = [(b"state"), user.key().as_slice(), state_bump.as_ref()];

    // NOTE: Different methods to derive the vault state PDA
    // Option 1: Using pubkey::checked_create_program_address => CUs Consumed: 5518
    // let state_seeds = &state_seed[..];
    // let state_pda = pubkey::checked_create_program_address(state_seeds, &crate::ID).unwrap();
    // log(&state_pda);

    // Option 2: Using pinocchio_pubkey::derive_address => CUs Consumed: 4142 (Preferred way!!)
    let state_pda = pinocchio_pubkey::derive_address(&state_seed, None, &crate::ID);
    log(&state_pda);
    assert_eq!(&state_pda, vault_state.key());

    // Validate vault PDA
    let vault_seed = [(b"vault"), state_pda.as_ref(), vault_bump.as_ref()];
    // let vault_seeds = &vault_seed[..];
    // let vault_pda = pubkey::checked_create_program_address(vault_seeds, &crate::ID).unwrap();
    let vault_pda = pinocchio_pubkey::derive_address(&vault_seed, None, &crate::ID);
    assert_eq!(&vault_pda, vault.key());

    // Prepare seeds for signing account creation
    let signer_seed = [Seed::from(b"state"), Seed::from(user.key().as_ref()), Seed::from(&state_bump)];
    let signers = Signer::from(&signer_seed);

    // Init-if-needed for vault state account
    if !vault_state.is_owned_by(&crate::ID) {
        CreateAccount {
            from: user,
            to: vault_state,
            lamports: Rent::get()?.minimum_balance(Vault::LEN),
            space: Vault::LEN as u64,
            owner: &crate::ID,
        }.invoke_signed(&[signers])?;

        let vault_state_account = Vault::from_account_info_mut(vault_state)?;
        vault_state_account.set_inner(vault_bump, state_bump);

        // Check that amount is greater than rent-exempt amount
        if amount < Rent::get()?.minimum_balance(0) {
            return Err(ProgramError::InvalidInstructionData);
        }

        msg!("Vault state account initialized");
        log(&vault_pda);
    }else{
        // Check that amount is greater than 0
        if amount == 0 {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    // Deposit SOL to vault
    Transfer {
        from: user,
        to: vault,
        lamports: amount,
    }.invoke()?;

    msg!("SOL deposited to vault");

    Ok(())
}