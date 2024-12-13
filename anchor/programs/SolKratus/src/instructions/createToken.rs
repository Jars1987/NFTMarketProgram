use {
    anchor_lang::{
        prelude::*, 
        solana_program::native_token::LAMPORTS_PER_SOL, 
        system_program
    }, 
    anchor_spl::{
        associated_token::{
            self, AssociatedToken
        },
        token::{
            self, Token,
        }
    }
};


#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub mint_account: Signer<'info>, //as per Metaplex: The Mint account of the asset. If it doesn't exist, it must be provided as a Signer as it will be initialized.
    pub mint_authority: Signer<'info>, // The authority of the Mint account. This is the account that is or will be allowed to mint tokens from the Mint account. This will default to the "Identity" wallet
 
    #[account(mut)]
       /// CHECK: Metaplex will check this
    pub token_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

pub fn create_token(ctx: &Context<CreateToken>) -> Result<()> {
   /*----------------- Step 1 - Create a Mint Account ---------------------------------- */

   msg!("Creating mint account...");

    let space = token::Mint::LEN as u64;
    let owner = &ctx.accounts.token_program.key();

    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = system_program::CreateAccount {
        from: ctx.accounts.mint_authority.to_account_info(),
        to: ctx.accounts.mint_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

   system_program::create_account(
       cpi_ctx,
       LAMPORTS_PER_SOL,
       space,
       owner,
   )?;

   /* -------------------- Step 2 - Initialize Mint Account ----------------------------- */
   msg!("Initializing mint account...");

   let cpi_program_mint = ctx.accounts.token_program.to_account_info();
   let cpi_program_acc = token::InitializeMint {
    mint: ctx.accounts.mint_account.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
   };

   let cpi_ctx_mint = CpiContext::new(
   cpi_program_mint,
   cpi_program_acc,
   );

   token::initialize_mint(
       cpi_ctx_mint,
       0,
       &ctx.accounts.mint_authority.key(),
       Some(&ctx.accounts.mint_authority.key()),
   )?;


   /* -------------------- Step 3 - Create Associated Token Account ---------------------- */
   msg!("Creating associated token account...");

   //instead of creating variables for everything you can just call it, example:

   associated_token::create(
    CpiContext::new(
        ctx.accounts.associated_token_program.to_account_info(),
        associated_token::Create {
            payer: ctx.accounts.mint_authority.to_account_info(),
            associated_token: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
            mint: ctx.accounts.mint_account.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
    ),
)?;
   /* -------------------- Step 4 - Mint Token to Associated Token Account --------------- */
   msg!("Minting token to token account...");

   token::mint_to(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        token::MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        },
    ),
    1,
)?;

   /* -------------------- Step 5 - Create Metadata Account ------------------------------ */
   // Check Metadata Instrucitons for this step. Instructions size to big to fit here.

   /* -------------------- Step 6 - Create Master Edition Account------------------------- */
       // Check Metadata Instrucitons for this step. Instructions size to big to fit here.

    Ok(())
}

