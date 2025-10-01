use bytemuck::{Pod, Zeroable};

use solana_program::pubkey::Pubkey;


#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]

pub struct Config
