use bytemuck::{Pod, Zeroable};

use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Config {
    pub version: u8,
    pub _padding1: [u8; 7], // padding to align admin field
    pub admin: Pubkey,
    pub fee: u64,
}

impl Config {
    pub const LEN: usize = 1 + 7 + 32 + 8; // version + padding + admin + fee
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct UserState {
    pub version: u8,
    pub _padding1: [u8; 7],
    pub balance: u64,
    pub owner: Pubkey,
}

impl UserState {
    pub const LEN: usize = 1 + 7 + 8 + 32; // byes calculation
}
