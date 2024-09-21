
use anchor_lang::prelude::*;
//1.this marketplace contract is not in any way related to the royalties distributed on the sale/purchase/distribution of royalties of a token
//2. this marketplace contract is only responsible for the sale and purchase of tokens associated with a particular NFT
//3. the royalties are distributed by the NFT contract
declare_id!("DLjJEsDLtUiJtMuHjurKhyba1YehvGNWXXWdM7jK3cBi");
mod contexts;
mod state;
use state::*;
use contexts::*;
#[program]
pub mod deauth_fractional_marketplace {
    use super::*;

  pub fn list_token(ctx : Context<ListToken>,price:u64, amount: u64)->Result<()> {
      // Figure out what the token account is (probably the token mint ie. the NFT's identification of being associated to this particular token)
      // check if the amount to be listed is less than the user currently owns
      Ok(())
  }

  pub fn buy_token(ctx : Context<BuyToken>, amount : u64) -> Result<()>{
    // check if the amount to be bought is less than the amount listed
    // check if the user has enough funds to buy the token
    // transfer the token to the user
    Ok(())
  }
  pub fn cancel_listing(ctx : Context<CancelListing>) -> Result<()>{
    // check if the user is the owner of the token
    // delete the token account ( what account ? ! ?)
    Ok(())
  }

  pub fn withdraw_proceeds(ctx: Context<WithdrawProceeds>)-> Result<()> {
    // check if the user is the owner of the token
    // transfer the funds to the user
    Ok(())
  }

  pub fn update_listing(ctx : Context<UpdateListing>) -> Result<()> {
    // check if the user is the owner of the token
    // update the price of the token
    Ok(())
  }

}

#[derive(Accounts)]
pub struct ListToken<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct TokenAccount {
    pub price: u64,
    pub amount: u64,
}
