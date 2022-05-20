use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("Authority address not matched with vesting term!")]
    MissMatchedAuthority,
    #[msg("Vesting account is immutable!")]
    VestingImmutable,
    #[msg("Claimer token account address not matched with signer!")]
    MissmatchedClaimTokenAccount,
    #[msg("Vesting token is miss matched!")]
    UnsupportedVestingToken,
    #[msg("Claimer info is not valid!")]
    InvalidClaimerInfo,
    #[msg("Authority already paused vesting process!")]
    PausedVesting,
    #[msg("Vesting time not started yet!")]
    UnstartedVesting,
}
