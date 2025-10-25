#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use litesvm::LiteSVM;

    use solana_instruction::{AccountMeta, Instruction};
    use solana_keypair::Keypair;
    use solana_message::Message;
    use solana_native_token::LAMPORTS_PER_SOL;
    use solana_pubkey::Pubkey;
    use solana_signer::Signer;
    use solana_transaction::Transaction;

    fn program_id() -> Pubkey {
        Pubkey::from(crate::ID)
    }

    // Setup the LiteSVM and the user keypair
    fn setup() -> (LiteSVM, Keypair) {
        let mut svm = LiteSVM::new();
        let user = Keypair::new();

        svm.airdrop(&user.pubkey(), 100 * LAMPORTS_PER_SOL).expect("Airdrop failed!");

        // Load program .so file
        msg!("The path is!! {}", env!("CARGO_MANIFEST_DIR"));
        let so_path = PathBuf::from("/Users/andreiacanadas/Documents/Solana/Github/pinocchio-vault/target/sbpf-solana-solana/release/pinocchio_vault.so");
        msg!("The path is!! {:?}", so_path);
    
        let program_data = std::fs::read(so_path).expect("Failed to read program SO file");
    
        svm.add_program(program_id(), &program_data);

        (svm, user)
    }

    #[test]
    fn test_first_deposit() {
        let (mut svm, user) = setup();

        let program_id = program_id();
        let system_program = solana_sdk_ids::system_program::ID;

        // Derive the vault state PDA:
        let (vault_state_pda, state_bump) = Pubkey::find_program_address(&[b"vault", user.pubkey().as_ref()], &program_id);

        // Derive the vault PDA:
        let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"state", vault_state_pda.as_ref()], &program_id);

        // Define amount to deposit:
        let amount = 10 * LAMPORTS_PER_SOL;

        // Create the deposit instruction:
        let deposit_data = [
            vec![0u8],
            vault_bump.to_le_bytes(),
            state_bump.to_le_bytes(),
            amount.to_le_bytes(),
        ].concat();
        let deposit_ix = Instruction{
            program_id,
            accounts: vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new(vault_state_pda, false),
                AccountMeta::new(system_program, false),
            ],
            data: deposit_data,
        };

        // Create and send the transaction containing the deposit instruction:
        let message = Message::new(&[deposit_ix], Some(&user.pubkey()));
        let recent_blockhash = svm.get_latest_blockhash().expect("Failed to get recent blockhash");

        let tx = Transaction::new(&[&[user]], message, recent_blockhash);

        // Log transaction details
        msg!("\n\nMake transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);
    }

    #[test]
    fn test_withdraw() {
        let (mut svm, user) = setup();
    }


}