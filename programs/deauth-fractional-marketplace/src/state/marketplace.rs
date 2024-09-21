use anchor_lang::prelude::*;

#[account]
pub struct MarketplaceState {
    pub owner: Pubkey,           // Owner of the marketplace (admin or business)
    pub seed: u64,      
    pub bump: u8,
    pub total_listings: u64,     // Total number of active listings
    pub fee_percentage: u64,     // Fee percentage for marketplace transactions
    pub royalty_percentage: u64, // Percentage for royalties (for the business)
}

impl Space for MarketplaceState {
    //first 8 bytes for the discriminator 
    const INIT_SPACE: usize = 64 + 8 + 1 + 8 + 8 + 8;
}

#[account]
pub struct MarketItem{
    pub owner : Pubkey,
    pub token_mint: Pubkey,
    pub price: u64,
    pub amount:u64
}
impl Space for MarketItem {
    //first 8 bytes for the discriminator 
    const INIT_SPACE: usize =8+ 32 + 32 + 8 + 8;
}

#[account]
pub struct FractionalOwner {
    pub owner: Pubkey,   // Public key of the fractional token owner
    pub shares_owned: u64,   // Number of fractional shares owned
    pub royalty_earned: u64,   // Royalties earned so far
    pub royalty_pending: u64,   // Royalties pending for distribution
}

#[account]
pub struct Token {
    
}