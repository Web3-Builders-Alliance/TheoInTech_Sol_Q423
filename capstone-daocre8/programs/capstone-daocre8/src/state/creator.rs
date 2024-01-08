use anchor_lang::prelude::*;

#[account]
pub struct Creator {
    pub signer: PubKey,
    pub seed: u64,
    pub bump: u8,
    pub project_dao_bump: u8,
}

impl Space for Creator {
    const INIT_SPACE: usize = 8 + 32 + 8 + 1 + 1;
}
