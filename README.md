# Pinocchio Vault
---
This is a simple Solana smart-contract to secure funds (SOL) in a vault PDA, built with Pinocchio library.

## What is it?
The Pinocchio Vault is a Solana program that allows users to deposit and withdraw SOL (lamports) into a vault account that is uniquely derived from each user, using a PDA. This ensures that the program is the only authority able to move the funds in and out the vault and that the owner is the only one that can access their vault.

- **Deposit:** Users can deposit a given amount of SOL into their vault PDA. If it is the first time, the vault will be created as a system account to hold the funds and a vault state account will also created to save the bumps. 
- **Withdraw:** Users can withdraw a given amount of SOL into their vault. If the remaining amount is inferior to the rent exempt lamports, all funds are witdrawn to user wallet and the accounts are closed.

## How it works?
- Each vault state account is a PDA derived from a static seed (b"state") and from the user's public key. This is a state account owned by the program to save the vault state bump and the vault bump.
- Each vault account is a PDA derived from a static seed (b"vault") and from the vault state account address. This is a system account to hold the SOL funds.
- First time the user deposits, it needs to ensure that enough lamports are being transferred to accomodate for rent exempt of the vault account.
- When withdrawing, if requested amount will leave the vault without enough lamports for rent exempt, then all SOL is transferred out of the vault and the accounts are closed.
- Once the accounts are closed the lamports on the state account are transferred back to the user.
- All transfers use Solana's system program for secure lamport movement.


## How it is implemented?
### Deposit
- The user signs a transaction to deposit a specified amount of lamports into their vault PDA.
- The program checks:
  - The owner is a signer.
  - The vault is owned by the system program.
  - The vault state account exists (is owned by the program). If not, it will create one and initialize it.
  - The vault state PDA is derived correctly.
  - The vault PDA is derived correctly.

### Withdraw
- The user signs a transaction a specified amount of lamports from their vault PDA.
- The program checks:
  - The owner is a signer.
  - The vault state PDA is derived correctly and owned by the program.
  - The vault PDA is derived correctly and owned by the system program.
  - The amount to be withdrawn. It it does not leave enough lamports, all vault balance is transferred to user and accounts are closed.
  - The program signs for the vault PDA using the correct seeds.
