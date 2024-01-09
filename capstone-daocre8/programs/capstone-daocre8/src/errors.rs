use anchor_lang::prelude::*;

#[error_code]
pub enum ProjectDAOError {
    #[msg("The project identifier already exists")]
    IdentifierAlreadyExists,

    #[msg("The project identifier is too long")]
    IdentifierTooLong,
}
