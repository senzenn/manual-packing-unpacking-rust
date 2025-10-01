use crate::{
    error::ProgramError,
    state::{Config, UserState},
};

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_error::ProgramError as SolanaProgramError,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instructions_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let config_account = next_account_info(account_info_iter)?;
    let user_account = next_account_info(account_info_iter)?;

    // pda derivation
    let (user_pda, _bump) =
        Pubkey::find_program_address(&[b"user", user_account.key.as_ref()], program_id);
    if *user_account.key != user_pda {
        // check pda matched or not
        return Err(ProgramError::InvalidPDA.into());
    }

    // Rent calculation
    let rent = Rent::get()?; // get rent async op 
    let config_lamports = rent.minimum_balance(Config::LEN); // Calculates minimum lamports for config account.
    if **config_account.lamports.borrow() < config_lamports {
        // Checks if config account has sufficient lamports.
        return Err(SolanaProgramError::InsufficientFunds);
    }

    // Deserialize config
    let config_slice = config_account.try_borrow_data()?;
    if config_slice.len() < Config::LEN {
        // Checks if data length is sufficient.
        return Err(SolanaProgramError::InvalidAccountData);
    }
    let config = bytemuck::from_bytes::<Config>(&config_slice[..Config::LEN]); // Deserializes config using bytemuck.

    if config.version != 1 {
        // Checks config version.
        return Err(ProgramError::InvalidVersion.into());
    }

    // Deserialize/update user
    let mut user_slice = user_account.try_borrow_mut_data()?; // Borrows mutable user account data, unwraps with ?.
    if user_slice.len() < UserState::LEN {
        // Checks if data length is sufficient.
        return Err(SolanaProgramError::InvalidAccountData);
    }
    let user = bytemuck::from_bytes_mut::<UserState>(&mut user_slice[..UserState::LEN]); // Deserializes user state.
    if user.version != 1 {
        // Checks user version.
        return Err(ProgramError::InvalidVersion.into());
    }
    user.balance += 1;

    Ok(())
}
