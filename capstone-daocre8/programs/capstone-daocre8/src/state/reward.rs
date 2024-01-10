use anchor_lang::prelude::*;

#[account]
pub struct Reward {
    pub project: PubKey,
    pub price: u64,
    pub reward_metadata: String,
    pub bump: u8,
}

impl Space for Reward {
    const INIT_SPACE: usize = 8 + 32 + 8 + (4 + 32) + 1;
}
