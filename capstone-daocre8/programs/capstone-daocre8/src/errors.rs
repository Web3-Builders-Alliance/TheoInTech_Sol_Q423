use anchor_lang::prelude::*;

#[error_code]
pub enum ProjectDAOError {
    #[msg("The project identifier already exists")]
    IdentifierAlreadyExists,

    #[msg("The project identifier is too long")]
    IdentifierTooLong,
}

#[error_code]
pub enum MilestoneError {
    #[msg("Milestone cannot be empty")]
    MilestoneCannotBeEmpty,
}

#[error_code]
pub enum RewardError {
    #[msg("Reward cannot be empty")]
    RewardCannotBeEmpty,
}
