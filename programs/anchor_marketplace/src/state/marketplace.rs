use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey, // 32 bytes
    pub fee: u16, // 2 bytes
    pub bump: u8, // 1 byte
    pub treasury_bump: u8, // 1 byte
    pub rewards_bump: u8, // 1 byte
    #[max_len(32)]
    pub name: String,
}
