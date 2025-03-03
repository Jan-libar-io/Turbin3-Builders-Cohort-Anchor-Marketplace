use anchor_lang::prelude::InterfaceAccount;
use anchor_spl::token::spl_token::state::Account;

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"marketplace".name.as_ref().as_bytes()],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    pub maker_mint_ata: InterfaceAccount<'info, TokenAccount>,
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub listing: Account<'info, Listing>,
    pub collection_mint: InterfaceAccount<'info, Mint>,
    pub metadata: Account<'info, MetadataAccount>,
    pub master_edition: Account<'info, MasterEditionAccount>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}