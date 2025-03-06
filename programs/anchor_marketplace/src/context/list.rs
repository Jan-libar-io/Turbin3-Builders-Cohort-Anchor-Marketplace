use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token_interface::{Mint, TokenAccount, TokenInterface},
    metadata::{
        Metadata,
        MetadataAccount,
        MasterEditionAccount
    }
};

use crate::state::{
    listing::Listing,
    marketplace::Marketplace
};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", marketplace.name.as_bytes()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = maker
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [b"listing", maker.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    pub metadata: Account<'info, MetadataAccount>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> List<'info> {

}
