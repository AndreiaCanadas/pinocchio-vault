#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use litesvm::LiteSVM;

    use litesvm_token::spl_token::solana_program::msg;
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
    fn test_vault() {
        let (mut svm, user) = setup();

        let program_id = program_id();
        let system_program = solana_sdk_ids::system_program::ID;

        // Derive the vault state PDA:
        let (vault_state_pda, state_bump) = Pubkey::find_program_address(&[b"state", user.pubkey().as_ref()], &program_id);
        msg!("Vault state PDA: {:?}", vault_state_pda);

        // Derive the vault PDA:
        let (vault_pda, vault_bump) = Pubkey::find_program_address(&[b"vault", vault_state_pda.as_ref()], &program_id);
        msg!("Vault PDA: {:?}", vault_pda);

        // TEST FIRST DEPOSIT (INITIALIZES VAULT)
        // Define amount to deposit:
        let amount = 10 * LAMPORTS_PER_SOL;

        // Create the deposit instruction:
        let deposit_data = [
            vec![0u8],  // Discriminator for deposit instruction
            vault_bump.to_le_bytes().to_vec(),
            state_bump.to_le_bytes().to_vec(),
            amount.to_le_bytes().to_vec(),
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
        let recent_blockhash = svm.latest_blockhash();

        let transaction = Transaction::new(&[&user], message, recent_blockhash);
        let tx = svm.send_transaction(transaction).unwrap();

        // Log transaction details
        msg!("\n\nDeposit (with init) transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);

        // ------------------------------------------------------------
        // TEST SECOND DEPOSIT
        // Define amount to deposit:
        let amount = 5 * LAMPORTS_PER_SOL;

        // Create the deposit instruction:
        let deposit_data = [
            vec![0u8],  // Discriminator for deposit instruction
            vault_bump.to_le_bytes().to_vec(),
            state_bump.to_le_bytes().to_vec(),
            amount.to_le_bytes().to_vec(),
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
        let recent_blockhash = svm.latest_blockhash();

        let transaction = Transaction::new(&[&user], message, recent_blockhash);
        let tx = svm.send_transaction(transaction).unwrap();

        // Log transaction details
        msg!("\n\nDeposit (without init) transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);

        // ------------------------------------------------------------
        // TEST WITHDRAW
        // Define amount to withdraw:
        let amount = 3 * LAMPORTS_PER_SOL;

        // Create the withdraw instruction:
        let withdraw_data = [
            vec![1u8],  // Discriminator for withdraw instruction
            amount.to_le_bytes().to_vec(),
        ].concat();
        let withdraw_ix = Instruction{
            program_id,
            accounts: vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new(vault_state_pda, false),
                AccountMeta::new(system_program, false),
            ],
            data: withdraw_data,
        };

        // Create and send the transaction containing the withdraw instruction:
        let message = Message::new(&[withdraw_ix], Some(&user.pubkey()));
        let recent_blockhash = svm.latest_blockhash();

        let transaction = Transaction::new(&[&user], message, recent_blockhash);
        let tx = svm.send_transaction(transaction).unwrap();

        // Log transaction details
        msg!("\n\nWithdraw transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);

        // ------------------------------------------------------------
        // TEST WITHDRAW ALL (CLOSING VAULT)
        // Define amount to withdraw:
        let amount = 12 * LAMPORTS_PER_SOL;

        // Create the withdraw instruction:
        let withdraw_data = [
            vec![1u8],  // Discriminator for withdraw instruction
            amount.to_le_bytes().to_vec(),
        ].concat();
        let withdraw_ix = Instruction{
            program_id,
            accounts: vec![
                AccountMeta::new(user.pubkey(), true),
                AccountMeta::new(vault_pda, false),
                AccountMeta::new(vault_state_pda, false),
                AccountMeta::new(system_program, false),
            ],
            data: withdraw_data,
        };

        // Create and send the transaction containing the withdraw instruction:
        let message = Message::new(&[withdraw_ix], Some(&user.pubkey()));
        let recent_blockhash = svm.latest_blockhash();

        let transaction = Transaction::new(&[&user], message, recent_blockhash);
        let tx = svm.send_transaction(transaction).unwrap();

        // Log transaction details
        msg!("\n\nWithdraw (closing vault) transaction sucessfull");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);
    }

}