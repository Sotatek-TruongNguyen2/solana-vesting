use crate::account::*;
use crate::context::*;
use crate::error::ProgramErrorCode::{
    InvalidClaimerInfo, PausedVesting, UnstartedVesting, VestingImmutable,
};
use crate::merkle_proof::*;
use crate::ID;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};

const VAULT_AUTHORITY_PDA_SEED: &[u8] = b"vault-authority";

#[program]
pub mod solana_staking {

    use super::*;

    pub fn initialize_vesting_term(
        ctx: Context<InitializeVestingTerm>,
        root: [u8; 32],
        is_mutable: bool,
        paused: bool,
    ) -> Result<()> {
        let clock: Clock = Clock::get().unwrap();

        let vesting_term: &mut Account<VestingAccountTerm> = &mut ctx.accounts.vesting_term;
        let authority: &Signer = &ctx.accounts.authority;
        let vault: &Account<TokenAccount> = &ctx.accounts.vault;
        let vesting_token: &Account<Mint> = &ctx.accounts.vesting_token;

        vesting_term.authority = authority.key();
        vesting_term.vesting_token = vesting_token.key();
        // vesting_term.vault = vault.key();
        vesting_term.is_mutable = is_mutable;
        vesting_term.paused = paused;
        vesting_term.root = root;
        vesting_term.start_vesting_at = clock.unix_timestamp;

        Ok(())
    }

    pub fn update_vesting_root_node(
        ctx: Context<UpdateVestingRootNode>,
        root: [u8; 32],
    ) -> Result<()> {
        let vesting_term: &mut Account<VestingAccountTerm> = &mut ctx.accounts.vesting_term;

        if !vesting_term.is_mutable {
            return Err(VestingImmutable.into());
        }

        vesting_term.root = root;

        Ok(())
    }

    pub fn claim(
        ctx: Context<Claim>,
        amount: u64,
        duration: i64,
        proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        let vesting_term: &Account<VestingAccountTerm> = &ctx.accounts.vesting_term;
        let vesting_account: &mut Account<VestingAccount> = &mut ctx.accounts.vesting_account;
        let claimer: &Signer = &ctx.accounts.claimer;

        if vesting_term.paused {
            return Err(PausedVesting.into());
        }

        // Verify the merkle proof.
        let node = anchor_lang::solana_program::keccak::hashv(&[
            &claimer.key().to_bytes(),
            &amount.to_le_bytes(),
            &duration.to_le_bytes(),
        ]);

        let valid_claimer = verify(proof, vesting_term.root, node.0);

        match valid_claimer {
            true => {
                let clock: Clock = Clock::get().unwrap();

                if !vesting_account.initialized {
                    vesting_account.initialized = true;
                    vesting_account.claimer = claimer.key();
                    vesting_account.total_amount = amount;
                    vesting_account.vesting_started_at = vesting_term.start_vesting_at;
                    vesting_account.last_vesting_at = clock.unix_timestamp;
                }

                if clock.unix_timestamp < vesting_account.vesting_started_at {
                    return Err(UnstartedVesting.into());
                }

                let vesting_percentage = clock
                    .unix_timestamp
                    .checked_sub(vesting_account.last_vesting_at)
                    .unwrap()
                    .checked_div(duration)
                    .unwrap();

                let claim_amount = vesting_account
                    .total_amount
                    .checked_mul(vesting_percentage.try_into().unwrap())
                    .unwrap();

                vesting_account.claim_amount = vesting_account
                    .claim_amount
                    .checked_add(claim_amount)
                    .unwrap();

                vesting_account.last_vesting_at = clock.unix_timestamp;
            }
            _ => return Err(InvalidClaimerInfo.into()),
        }

        Ok(())
    }
}
