use anchor_lang::prelude::*;
use crate::state::marketplace::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space =  64 + 8 + 1 + 8 + 8 + 8)]
    pub marketplace :Account<'info, MarketplaceState>,  
    #[account(mut)]
    pub user: Signer<'info>, //has to be the business only who initializes the marketplace 
    pub system_program: Program<'info, System>,
}
