use crate::account::*;
use crate::error::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, Mint, TokenAccount};

#[derive(Accounts)]
pub struct InitializeVestingTerm<'info> {
    #[account(
        init, 
        payer = authority, 
        space = VestingAccountTerm::LEN, 
        has_one = authority @ProgramErrorCode::MissMatchedAuthority
    )]
    pub vesting_term: Account<'info, VestingAccountTerm>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            b"vault",
            vesting_term.key().as_ref()
        ],
        bump,
        payer = authority,
        token::authority = vesting_term,
        token::mint = vesting_token
    )]
    pub vault: Account<'info, TokenAccount>,
    pub vesting_token: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateVestingRootNode<'info> {
    #[account(
        has_one = authority @ProgramErrorCode::MissMatchedAuthority
    )]
    pub vesting_term: Account<'info, VestingAccountTerm>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    pub vesting_term: Account<'info, VestingAccountTerm>,
    #[account( 
        init_if_needed,
        seeds = [
            b"vesting_account",
            vesting_term.key().as_ref(),
            claimer.key().as_ref()
        ],
        space = VestingAccount::LEN,
        payer = claimer,
        bump,
        has_one = claimer
    )]
    pub vesting_account: Account<'info, VestingAccount>,

    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        owner = Token::id(),
        constraint = claimer_token_account.owner.key() == claimer.key() @ProgramErrorCode::MissmatchedClaimTokenAccount,
        constraint = claimer_token_account.mint.key() == vesting_term.vesting_token.key() @ProgramErrorCode::UnsupportedVestingToken,
    )]
    pub claimer_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
