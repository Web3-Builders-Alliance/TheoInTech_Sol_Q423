use anchor_lang::prelude::*;

#[account]
pub struct Rewards {
    pub project: PubKey,
    pub price: u64,
    pub metadata: String,
    pub bump: u8,
}

impl Space for Rewards {
    const INIT_SPACE: usize = 8 + 32 + 8 + (4 + 32) + 1;
}
