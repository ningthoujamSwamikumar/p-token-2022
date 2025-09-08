use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

use crate::state::{COption, Initializable, Transmutable};

#[repr(C)]
pub struct Mint {
    mint_authority: COption<Pubkey>,
    supply: [u8; 8],
    pub decimals: u8,
    is_initialized: u8,
    freeze_authority: COption<Pubkey>,
}

impl Mint {
    #[inline(always)]
    pub fn set_supply(&mut self, supply: u64) {
        self.supply = supply.to_le_bytes();
    }

    #[inline(always)]
    pub fn supply(&self) -> u64 {
        u64::from_le_bytes(self.supply)
    }

    #[inline(always)]
    pub fn set_initialized(&mut self) {
        self.is_initialized = 1;
    }

    #[inline(always)]
    pub fn clear_mint_authority(&mut self) {
        self.mint_authority.0[0] = 0;
    }

    #[inline(always)]
    pub fn set_mint_authority(&mut self, mint_authority: &Pubkey) {
        self.mint_authority.0[0] = 1;
        self.mint_authority.1 = *mint_authority;
    }

    #[inline(always)]
    pub fn mint_authority(&self) -> Option<&Pubkey> {
        if self.mint_authority.0[0] == 1 {
            Some(&self.mint_authority.1)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn clear_freeze_authority(&mut self) {
        self.freeze_authority.0[0] = 0;
    }

    #[inline(always)]
    pub fn set_freeze_authority(&mut self, freeze_authority: &Pubkey) {
        self.freeze_authority.0[0] = 1;
        self.freeze_authority.1 = *freeze_authority;
    }

    pub fn freeze_authority(&self) -> Option<&Pubkey> {
        if self.freeze_authority.0[0] == 1 {
            Some(&self.freeze_authority.1)
        } else {
            None
        }
    }
}

unsafe impl Transmutable for Mint {
    const LEN: usize = size_of::<Mint>();
}

impl Initializable for Mint {
    fn is_initialized(&self) -> Result<bool, pinocchio::program_error::ProgramError> {
        match self.is_initialized {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}
