pub mod error;
use crate::error::ErrorCode;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, InitializeMint, Mint, Token, TokenAccount, Transfer};
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("82547nhsM6oQdyDdNNiHvZEnZHRbzVfq9CKb2jUYaqaS");

#[program]
mod deauth_fractional_nft {
    use super::*;

    pub fn initialize_nft(ctx: Context<InitializeNft>, name: String) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.name = name;
        nft.total_shares = 1000;
        nft.business_shares = 300; // Business reserves 30% ownership
        nft.public_shares = 700; // Remaining 70% for sale
        nft.creator = ctx.accounts.creator.key();

        Ok(())
    }

    pub fn buy_fractions(
        ctx: Context<BuyFractions>,
        num_shares: u64,
        price_per_share: u64,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        let fractional_owner = &mut ctx.accounts.fractional_owner;

        // Ensure there are enough shares available for purchase
        require!(
            nft.public_shares >= num_shares,
            ErrorCode::NotEnoughSharesAvailable
        );

        // Deduct the shares from the NFT's available public shares
        nft.public_shares -= num_shares;

        // Add shares to the user's ownership
        fractional_owner.shares_owned += num_shares;

        // Handle payment logic (to business, seller, etc.)
        // Transfer payment from buyer to relevant accounts

        Ok(())
    }
    pub fn sell_fractions(
        ctx: Context<SellFractions>,
        num_shares: u64,
        price_per_share: u64,
    ) -> Result<()> {
        let fractional_owner = &mut ctx.accounts.fractional_owner;

        // Ensure the seller has enough shares to sell
        require!(
            fractional_owner.shares_owned >= num_shares,
            ErrorCode::NotEnoughSharesOwned
        );

        // Deduct the shares from the seller
        fractional_owner.shares_owned -= num_shares;

        // Handle payment logic and transfer of shares to the buyer
        // Payment can include a small royalty fee that goes to the business/creator

        Ok(())
    }
    pub fn distribute_royalties(
        ctx: Context<DistributeRoyalties>,
        total_royalty: u64,
    ) -> Result<()> {
        let nft = &ctx.accounts.nft;
        let fractional_owner = &mut ctx.accounts.fractional_owner;

        // Calculate share of royalties based on ownership
        let share_of_royalty = (fractional_owner.shares_owned * total_royalty) / nft.total_shares;

        // Send the calculated royalty to the fractional owner
        // Transfer logic here

        Ok(())
    }
    pub fn update_sales_data(ctx: Context<UpdateSalesData>, sales_amount: u64) -> Result<()> {
        let nft = &mut ctx.accounts.nft;

        // Logic to handle updating sales data
        // Sales amount will be used in royalty distribution

        Ok(())
    }
    pub fn withdraw_royalties(ctx: Context<WithdrawRoyalties>, amount: u64) -> Result<()> {
        let fractional_owner = &mut ctx.accounts.fractional_owner;

        // Ensure the owner has enough accumulated royalties
        require!(
            fractional_owner.royalty_pending >= amount,
            ErrorCode::NoRoyaltiesToClaim
        );

        // Deduct the royalties from the owner's balance
        fractional_owner.royalty_pending -= amount;

        // Transfer the royalties to the receiver
        // Transfer logic here

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeNft<'info> {
    #[account(init, payer = creator, space = 8 + 256)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut)]
    pub business: Signer<'info>, // Business that owns 30% of the shares
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Nft {
    pub name: String,
    pub total_shares: u64,    // Total shares (e.g. 1000 shares for 100%)
    pub business_shares: u64, // 30% reserved for business
    pub public_shares: u64,   // 70% for users to buy
    pub creator: Pubkey,      // Address of the creator
}
#[derive(Accounts)]
pub struct BuyFractions<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub fractional_owner: Account<'info, FractionalOwner>,
}

#[account]
pub struct FractionalOwner {
    pub owner: Pubkey,
    pub shares_owned: u64,   // Number of shares owned by the user
    pub royalty_earned: u64, // Total royalties earned so far
    pub royalty_pending: u64,
}

#[derive(Accounts)]
pub struct SellFractions<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub fractional_owner: Account<'info, FractionalOwner>,
}
#[derive(Accounts)]
pub struct DistributeRoyalties<'info> {
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    #[account(mut)]
    pub fractional_owner: Account<'info, FractionalOwner>,
    #[account(mut)]
    pub business: Signer<'info>, // Business that receives 30% of royalties
    #[account(mut)]
    pub creator: Signer<'info>, // Creator of the design
}

#[derive(Accounts)]
pub struct UpdateSalesData<'info> {
    #[account(mut)]
    pub nft: Account<'info, Nft>,
    pub authority: Signer<'info>, // Only trusted authority (oracle) can call this
}
#[derive(Accounts)]
pub struct WithdrawRoyalties<'info> {
    #[account(mut)]
    pub fractional_owner: Account<'info, FractionalOwner>,
    #[account(mut)]
    pub receiver: Signer<'info>, // The account receiving the withdrawn funds
}
