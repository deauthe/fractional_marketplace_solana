use anchor_lang::prelude::*;
use crate::state::{MarketPlaceState};


#[derive(Accounts)]
pub struct CreateListing<'info> {
    #[account(mut)]
    pub marketplace: Account<'info, MarketPlaceState>,
    #[account(init, payer = seller),space = 8]
    pub market_item : Account<'info,MarketItem>

}