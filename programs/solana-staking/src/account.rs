use anchor_lang::prelude::*;

#[account]
pub struct VestingAccount {
    pub claimer: Pubkey,
    pub vesting_started_at: i64,
    pub last_vesting_at: i64,
    pub total_amount: u64,
    pub claim_amount: u64,
    pub initialized: bool,
    pub is_freezed: bool,
}

#[account]
pub struct VestingAccountTerm {
    /// The 256-bit merkle root.
    pub root: [u8; 32],
    pub authority: Pubkey,
    pub vesting_token: Pubkey,
    pub distributor: Pubkey,
    pub start_vesting_at: i64,
    pub is_mutable: bool,
    pub paused: bool,
}

// 2. Add some useful constants for sizing propeties.
const BOOL_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const DISCRIMINATOR_LENGTH: usize = 8;
const TIMESTAMP_LENGTH: usize = 64;
const TOKENS_AMOUNT_LENGTH: usize = 64;

impl VestingAccount {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + BOOL_LENGTH * 2
        + TOKENS_AMOUNT_LENGTH * 2
        + TIMESTAMP_LENGTH * 2;
}

impl VestingAccountTerm {
    pub const LEN: usize =
        DISCRIMINATOR_LENGTH + PUBLIC_KEY_LENGTH * 3 + BOOL_LENGTH * 2 + TIMESTAMP_LENGTH;
}
