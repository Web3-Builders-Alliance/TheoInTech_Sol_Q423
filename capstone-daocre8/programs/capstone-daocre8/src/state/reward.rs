use anchor_lang::prelude::*;

#[account]
pub struct Reward {
    pub reward_idx: String,
    pub project: Pubkey,
    pub price: u64,
    pub vote_weight: u8,
    pub reward_metadata: String,
    pub number_of_backers: u64,
    pub bump: u8,
}

impl Space for Reward {
    const INIT_SPACE: usize = 8 + (4 + 32) + 32 + 8 + 1 + (4 + 32) + 8 + 1;
}

#[account]
pub struct RewardBacked {
    pub backer: Pubkey,
    pub bump: u8,
}

impl Space for RewardBacked {
    const INIT_SPACE: usize = 8 + 32 + 1;
}
