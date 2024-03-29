use anchor_lang::prelude::*;

#[account]
pub struct Backer {
    pub signer: Pubkey,
    pub seed: u64,
    pub bump: u8,
    pub project_dao_bump: u8,
}

impl Space for Backer {
    const INIT_SPACE: usize = 8 + 32 + (4 + 32) + 8 + 1 + 1;
}
