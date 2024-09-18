use anchor_lang::prelude::*;
#[error_code]
pub enum ErrorCode {
    #[msg("Not enough shares available.")]
    NotEnoughSharesAvailable,
    #[msg("Not enough shares owned to sell.")]
    NotEnoughSharesOwned,
    #[msg("Insufficient royalties to withdraw.")]
    NoRoyaltiesToClaim,
}
