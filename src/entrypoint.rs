use crate::{error::TokenError, processor::process_initialize_mint};
use pinocchio::{
    ProgramResult, account_info::AccountInfo, no_allocator, nostd_panic_handler,
    program_entrypoint, pubkey::Pubkey,
};

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
) -> ProgramResult {
    let [discriminator, remaining @ ..] = instruction_data else {
        return Err(TokenError::InvalidInstruction.into());
    };

    match *discriminator {
        // 0 - InitializeMint
        0 => {
            #[cfg(feature = "logging")]
            pinocchio::msg!("Instruction: InitializeMint");

            process_initialize_mint(accounts, remaining)
        }
        // 20 - InitializeMint2
        20 => process_initialize_mint(accounts, remaining),
        _ => Err(TokenError::InvalidInstruction.into()),
    }
}
