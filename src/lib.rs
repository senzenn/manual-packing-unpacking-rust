// export all the  modules

pub mod entrypoint;
pub mod state;
pub mod error;
pub mod instruction;

use crate::entrypoint::process_instruction;
solana_program::solana_program_entrypoint!(process_instruction);
