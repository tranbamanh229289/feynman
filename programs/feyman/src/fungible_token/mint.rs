use anchor_lang::prelude::*;
use anchor_spl::{token, associated_token};

#[program]
pub mod mint {

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        seeds = [b"mint", user.key().as_ref()],
        bump,
        payer = user,
        mint::decimals = 9,
        mint::authority = mint,
    )]
    pub mint: Account<'info, token::Mint>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub token_account: Account<'info, token::TokenAccount>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}