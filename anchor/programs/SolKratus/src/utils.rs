use anchor_lang::prelude::*;


use crate::{
  constants::{EDITION, METADATA},
};




pub fn token_metadata_program_id() -> Pubkey {
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        .parse()
        .unwrap()
}

pub fn find_master_edition_account(mint: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
      &[
          METADATA.as_bytes(),
          token_metadata_program_id().as_ref(),
          mint.as_ref(),
          EDITION.as_bytes(),
      ],
      &token_metadata_program_id(),
  )
}

pub fn find_metadata_account(mint: &Pubkey) -> (Pubkey, u8) {
  Pubkey::find_program_address(
      &[
          METADATA.as_bytes(),
          token_metadata_program_id().as_ref(),
          mint.as_ref(),
      ],
      &token_metadata_program_id(),
  )
}