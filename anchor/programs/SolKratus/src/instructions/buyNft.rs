use {
  anchor_lang::{
      prelude::*, system_program,
  },
  anchor_spl::{
      associated_token::AssociatedToken,
      token::{self, Token, TokenAccount, Mint},
  },
};

use crate::state::*;
use crate::error::ErrorCode;


pub fn buy(
  ctx: Context<BuyNft>,
) -> Result<()> {
  let nft_listing_account = &mut ctx.accounts.nft_listing_account;

  msg!("Buying NFT");
  // Ensure the buyer has sent enough lamports
  require!(
      ctx.accounts.buyer.lamports() >= nft_listing_account.price,
      ErrorCode::InsufficientFunds
  );

  let bump_seed = ctx.bumps.vault_pda;
  let vault_pda_seeds : &[&[&[u8]]] = &[&[NFT_MARKET_PLACE_SEED.as_bytes(), &[bump_seed]]];

  system_program::transfer(
      CpiContext::new(
          ctx.accounts.system_program.to_account_info(),
          system_program::Transfer {
              from: ctx.accounts.buyer.to_account_info(),
              to: ctx.accounts.seller.to_account_info(),
          }
      ),
      nft_listing_account.price,
  )?;
  msg!("Lamports transferred successfully.");

  // // Transfer payment from buyer to seller
  **ctx.accounts.buyer.lamports.borrow_mut() -= nft_listing_account.price;
  **ctx.accounts.seller.lamports.borrow_mut() += nft_listing_account.price;

  // Transfer NFT from PDA to buyer
  token::transfer(
      CpiContext::new(
          ctx.accounts.token_program.to_account_info(),
          token::Transfer {
              from: ctx.accounts.vault_token_account.to_account_info(),
              to: ctx.accounts.buyer_token_account.to_account_info(),
              authority: ctx.accounts.vault_pda.to_account_info(),
          })
      .with_signer(vault_pda_seeds),
      1,
  )?;

  msg!("NFT purchased successfully!");

  Ok(())
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
  #[account(mut)]
  pub buyer: Signer<'info>,

  /// CHECK: The seller's account
  #[account(mut)]
  pub seller: UncheckedAccount<'info>,

  #[account(mut)]
  pub mint: Account<'info, Mint>,

  #[account(
      mut,
      seeds = [
          LISTED_NFT_SEED.as_bytes(),
          mint.key().as_ref()
      ],
      bump,
      close = seller,
  )]
  pub nft_listing_account: Account<'info, ListedNft>,

  #[account(
      mut,
      associated_token::mint = mint,
      associated_token::authority = vault_pda,
  )]
  pub vault_token_account: Account<'info, TokenAccount>,

  #[account(
      init_if_needed,
      payer = buyer,
      associated_token::mint = mint,
      associated_token::authority = buyer,
  )]
  pub buyer_token_account: Account<'info, TokenAccount>,

  /// CHECK: Program Derived Address
  #[account(
      seeds = [NFT_MARKET_PLACE_SEED.as_bytes()],
      bump,
  )]
  pub vault_pda: UncheckedAccount<'info>,

  pub token_program: Program<'info, Token>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,
  pub rent: Sysvar<'info, Rent>,
}