use anchor_lang::prelude::*;

use crate::context::*;

declare_id!("8nW9xCyKhegYye8mFSuKxtbUZnyPHYw2ipjdwxNytC3w");

pub mod state;
pub mod context;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.initialize_marketplace(name, fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn list(ctx: Context<List>) -> Result<()> {
        Ok(())
    }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     Ok(())
    // }
    
    // pub fn delist(ctx: Context<Delist>) -> Result<()> {
    //     Ok(())
    // }
}
