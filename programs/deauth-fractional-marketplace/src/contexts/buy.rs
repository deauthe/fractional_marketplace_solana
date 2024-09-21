use anchor_lang::prelude::*;

use crate::{marketplace::MarketplaceState, TokenAccount};

// Replace with your actual program ID
pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,  // Buyer's public key
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,  // Buyer's token account
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,  // Seller's token account
    #[account(mut)]
    pub marketplacestate: Account<'info, MarketplaceState>,  // Marketplace account
    #[account(mut)]
    pub payment_account: Account<'info, TokenAccount>,  // Payment account
    #[account(address = TOKEN_PROGRAM_ID)]
    pub token_program: Program<'info, Token>,  // Token program for SPL token interaction
    pub system_program: Program<'info, System>,
}
