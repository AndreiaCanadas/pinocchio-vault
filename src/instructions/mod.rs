pub mod deposit;
pub use deposit::*;

pub mod withdraw;
pub use withdraw::*;

use pinocchio::program_error::ProgramError;

pub enum VaultInstructions {
    Deposit = 0,
    Withdraw = 1,
}

impl TryFrom<u8> for VaultInstructions {
    type Error = ProgramError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(VaultInstructions::Deposit),
            1 => Ok(VaultInstructions::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}