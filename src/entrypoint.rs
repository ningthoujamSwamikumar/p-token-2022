use pinocchio::{
    account_info::AccountInfo, 
    no_allocator, 
    nostd_panic_handler, 
    program_entrypoint, 
    pubkey::Pubkey, 
    ProgramResult
};
use crate::error::TokenError;



program_entrypoint!(process_instruction);
// do not allocate memory
no_allocator!();
// use the no_std panic handler
nostd_panic_handler!();

/// Process an instruction
/// 
#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult {
    let [discriminator, remaining@..] = instruction_data else {
        return Err(TokenError::InvalidInstruction.into());
    };

    // match *discriminator {
    //     0 => 
    // }

    Ok(())
}

