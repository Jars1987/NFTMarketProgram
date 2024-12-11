#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod SolKratus {
    use super::*;

  pub fn close(_ctx: Context<CloseSolKratus>) -> Result<()> {
    Ok(())
  }

  pub fn decrement(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.SolKratus.count = ctx.accounts.SolKratus.count.checked_sub(1).unwrap();
    Ok(())
  }

  pub fn increment(ctx: Context<Update>) -> Result<()> {
    ctx.accounts.SolKratus.count = ctx.accounts.SolKratus.count.checked_add(1).unwrap();
    Ok(())
  }

  pub fn initialize(_ctx: Context<InitializeSolKratus>) -> Result<()> {
    Ok(())
  }

  pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
    ctx.accounts.SolKratus.count = value.clone();
    Ok(())
  }
}

#[derive(Accounts)]
pub struct InitializeSolKratus<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  init,
  space = 8 + SolKratus::INIT_SPACE,
  payer = payer
  )]
  pub SolKratus: Account<'info, SolKratus>,
  pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseSolKratus<'info> {
  #[account(mut)]
  pub payer: Signer<'info>,

  #[account(
  mut,
  close = payer, // close account and return lamports to payer
  )]
  pub SolKratus: Account<'info, SolKratus>,
}

#[derive(Accounts)]
pub struct Update<'info> {
  #[account(mut)]
  pub SolKratus: Account<'info, SolKratus>,
}

#[account]
#[derive(InitSpace)]
pub struct SolKratus {
  count: u8,
}
