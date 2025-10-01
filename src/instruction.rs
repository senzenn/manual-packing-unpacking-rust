use crate::{
    error::ProgramError,
    state::{Config, UserState},
};

use bytemuck::Pod;

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError as SoalanError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn process_instruction(
    program_id: &[Pubkey],
    accounts: &[AccountInfo],
    _instructions_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_account = next_account_info(account_info_iter);
    let user_account = next_account_info(account_info_iter);

    // pda derivation
    let (user_pda, _bump) =
        Pubkey::find_program_address(&[b"user", user_account.key.as_ref()], program_id); // Derives PDA and bump seed.
    if *user_account.key != user_pda {
        // Checks if user account key matches derived PDA.
        return Err(ProgramError::InvalidPDA.into()); // Returns error if mismatch.
    }
}
