use bytemuck::{Pod, Zeroable};

use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]

pub struct Config {
    pub version: u8,
    pub admin: Pubkey,
    pub fee: u64,
}

impl Config {
    pub const LEN: usize = 1 + 32 + 8;
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UserState {
    pub version: u8,
    pub balance: u64,
    pub owner: Pubkey,
}

impl UserState {
    pub const LEN: usize = 1 + 8 + 32;
}
