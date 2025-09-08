use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{Sysvar, rent::Rent},
};

use crate::{
    error::TokenError,
    state::{Initializable, load_mut_unchecked, mint::Mint},
};

#[inline(always)]
fn _process_initialize_mint(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
    rent_sysvar_account: bool,
) -> ProgramResult {
    // The minimum size of instruction data is either 34 or 66 bytes
    // - decimals (1)
    // - mint_authority (32)
    // - option + freeze_authority (1 + 32) bytes
    if instruction_data.len() < 34 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let (decimals, mint_authority, freeze_authority) = unsafe {
        let decimals = instruction_data.get_unchecked(0);
        let mint_authority = &*(instruction_data.as_ptr().add(1) as *const Pubkey);
        let freeze_authority = match *instruction_data.get_unchecked(33) {
            0 => None,
            1 => {
                if instruction_data.len() < 66 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                Some(&*(instruction_data.as_ptr().add(34) as *const Pubkey))
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        };
        (decimals, mint_authority, freeze_authority)
    };

    //validate the accounts
    let (mint_info, rent_sysvar_info) = if rent_sysvar_account {
        let [mint_info, rent_sysvar_info, _remaining @ ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        (mint_info, Some(rent_sysvar_info))
    } else {
        let [mint_info, _remaining @ ..] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };
        (mint_info, None)
    };

    let mint_data_len = mint_info.data_len();

    let is_exempt = if let Some(rent_sysvar_info) = rent_sysvar_info {
        let rent = unsafe { Rent::from_account_info_unchecked(rent_sysvar_info)? };
        rent.is_exempt(mint_info.lamports(), mint_data_len)
    } else {
        Rent::get()?.is_exempt(mint_info.lamports(), mint_data_len)
    };

    if !is_exempt {
        return Err(ProgramError::AccountNotRentExempt);
    }

    let mint = unsafe { load_mut_unchecked::<Mint>(mint_info.borrow_mut_data_unchecked())? };

    if mint.is_initialized()? {
        return Err(TokenError::AlreadyInUse.into());
    };

    // initialize the mint
    mint.set_initialized();
    mint.set_mint_authority(mint_authority);
    mint.decimals = *decimals;

    if let Some(freeze_authority) = freeze_authority {
        mint.set_freeze_authority(freeze_authority);
    };

    Ok(())
}

pub fn process_initialize_mint(accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    _process_initialize_mint(accounts, instruction_data, true)
}

pub fn process_initialize_mint2(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    _process_initialize_mint(accounts, instruction_data, false)
}
