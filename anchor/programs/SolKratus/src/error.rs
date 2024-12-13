use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Your have insuficient funds")]
    InsufficientFunds,
}