use {
  anchor_lang::prelude::*,
  anchor_spl::{
      token::Token,
      metadata::{
          create_metadata_accounts_v3,
          CreateMetadataAccountsV3,
          create_master_edition_v3,
          CreateMasterEditionV3,
          Metadata,
          mpl_token_metadata::types::{
              DataV2, Creator
          },
      },
  }
};




#[derive(Accounts)]
pub struct CreateMetadata<'info> {
    #[account(mut)]
    pub mint_account: Signer<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(
      mut,
      seeds = [b"metadata".as_ref(), token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key()
  )]
  /// CHECK: Metaplex will do the check
   pub metadata: UncheckedAccount<'info>,
   #[account(
      mut,
      seeds = [b"metadata".as_ref(), token_metadata_program.key().as_ref(), mint_account.key().as_ref(), b"edition".as_ref()],
      bump,
      seeds::program = token_metadata_program.key()
  )]
  /// CHECK:
    pub master_edition: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,

}

pub fn create_metadata( context: Context<CreateMetadata>, title: String, uri: String, symbol: String) -> Result<()> {
  msg!("Creating metadata account...");
  
  create_metadata_accounts_v3(
      CpiContext::new(
          context.accounts.metadata.to_account_info(),
          CreateMetadataAccountsV3{
              metadata: context.accounts.metadata.to_account_info(),
              mint: context.accounts.mint_account.to_account_info(),
              payer: context.accounts.mint_authority.to_account_info(),
              mint_authority: context.accounts.mint_authority.to_account_info(),
              update_authority: context.accounts.mint_authority.to_account_info(),
              system_program: context.accounts.system_program.to_account_info(),
              rent: context.accounts.rent.to_account_info(),
          }
      ),
      DataV2 {
          name: title,
          symbol: symbol,
          uri: uri,
          seller_fee_basis_points: 0,
          creators: Some(vec![
              Creator {
                  address: context.accounts.mint_authority.key(),
                  verified: true,
                  share: 100,
              },
          ]),
          collection: None,
          uses: None,
      },
      true,
      true,
      None,
  )?;

  msg!("Creating master edition metadata account...");
  
  create_master_edition_v3(
      CpiContext::new(
          context.accounts.token_metadata_program.to_account_info(),
          CreateMasterEditionV3{
              edition: context.accounts.master_edition.to_account_info(),
              payer: context.accounts.mint_authority.to_account_info(),
              mint: context.accounts.mint_account.to_account_info(),
              metadata: context.accounts.metadata.to_account_info(),
              mint_authority: context.accounts.mint_authority.to_account_info(),
              update_authority: context.accounts.mint_authority.to_account_info(),
              system_program: context.accounts.system_program.to_account_info(),
              token_program: context.accounts.token_program.to_account_info(),
              rent: context.accounts.rent.to_account_info(),
          }),
          Some(1),
  )?;

  msg!("Minted NFT successfully");

  Ok(())
}