use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub backer_address: Pubkey,
    pub creator_address: Pubkey,
    pub receive: u64,
    pub bump: u8,
}

impl Space for Treasury {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
