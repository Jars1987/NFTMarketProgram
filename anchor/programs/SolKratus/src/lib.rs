pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use utils::*;
pub use state::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod SolKratus {
    use super::*;
    pub fn mint(
      ctx: Context<CreateToken>,
  ) -> Result<()> {
      createToken::create_token(&ctx)

  }

  pub fn create_metadata(
    ctx: Context<CreateMetadata>,
    title: String,
    uri: String,
    symbol: String,
) -> Result<()> {
    createMetadata::create_metadata(ctx, title, uri, symbol)
}


pub fn list_nft(
    ctx: Context<ListNft>,
    price: u64,
) -> Result<()> {
    listNft::list_nft(ctx, price)
}

}