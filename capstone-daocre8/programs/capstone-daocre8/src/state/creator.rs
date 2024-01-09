use anchor_lang::prelude::*;

#[account]
pub struct Creator {
    pub signer: PubKey,
    pub project_dao_count: u8,
    pub bump: u8,
    pub project_dao_bump: u8,
}

impl Space for Creator {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1 + 1;
}
