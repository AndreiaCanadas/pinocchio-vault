use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vault {
    pub vault_bump: [u8; 1],
    pub state_bump: [u8; 1],
}
impl Vault {
    pub const LEN: usize = 2;

    pub fn from_account_info(account_info: &AccountInfo) -> Result<&Self, ProgramError> {
        let data = account_info.try_borrow_data()?;
        if data.len() != Vault::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { & *(data.as_ptr() as *const Self) })
    }

    pub fn from_account_info_mut(account_info: &AccountInfo) -> Result<&mut Self, ProgramError> {
        let mut data = account_info.try_borrow_mut_data()?;
        if data.len() != Vault::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        Ok(unsafe { &mut *(data.as_mut_ptr() as *mut Self) })
    }

    pub fn set_inner(&mut self, vault_bump: [u8; 1], state_bump: [u8; 1]) {
        self.vault_bump = vault_bump;
        self.state_bump = state_bump;
    }
}