use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const TEXT_LENGTH: usize = 1024;
const USER_NAME_LENGTH: usize = 100;
const USER_URL_LENGTH: usize = 255;

#[program]
pub mod programs {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateState<'info> {
    #[account(
        init,
        seeds = [b"state".as_ref()], // automatically generate a random seed
        bump, // to always have a unique id
        payer = authority,
        space = size_of::<StateAccount>() + 8
    )]
    pub state: Account<'info, StateAccount>,

    // authority (this is signer who paid transaction fee)
    #[account(mut)]
    pub authority: Signer<'info>,

    // system program
    pub system_program: UncheckedAccount<'info>,

    // token program
    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(
        mut,
        seeds = [b"state".as_ref(), bump]
    )]
    pub state: Account<'info, StateAccount>,

    // authenticate post account
    #[account(
        init,
        // post account use "post" and index of post as seed
        seeds = [b"post".as_ref(), state.post_count.to_be_bytes().as_ref()],
        bump,
        payer = authority,
        space = size_of::<PostAccount>() + USER_URL_LENGTH + TEXT_LENGTH + USER_NAME_LENGTH
    )]
    pub post: Account<'info, PostAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: UncheckedAccount<'info>,

    #[account(constraint = token_program.key == &token::ID)]
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct StateAccount {
    pub authority: Pubkey,
    pub post_count: u64,
}

// Post Account Structure
#[account]
pub struct PostAccount {
    pub authority: Pubkey, // signer address
    pub text: String,
    pub: poster_name: String,
    pub: poster_url: String,
    pub: comment_count: u64,
    pub index: u64,
    pub post_time: i64,
}