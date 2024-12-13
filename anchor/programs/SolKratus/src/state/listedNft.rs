use anchor_lang::prelude::*;

pub const NFT_MARKET_PLACE_SEED: &str = "NFT_MARKETPLACE_EMSKIQ";
pub const LISTED_NFT_SEED: &str = "LISTED_NFT_EMSKIQ_SEED";

#[account]
pub struct ListedNft {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub price: u64,
}

impl ListedNft {
    pub const MAX_SIZE: usize = 32 + 32 + 8;
}