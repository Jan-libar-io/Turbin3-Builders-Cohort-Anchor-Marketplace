use anchor_lang::prelude::*;

#[account]
pub struct Listing {
    pub maker: Pubkey, // 32 bytes
    pub mint: Pubkey, // 32 bytes
    pub price: u64, // 1 byte
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
