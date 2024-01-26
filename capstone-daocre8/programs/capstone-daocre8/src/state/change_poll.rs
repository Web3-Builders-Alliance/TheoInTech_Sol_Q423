use anchor_lang::prelude::*;

#[account]
pub struct ChangePoll {
    pub change_poll_idx: String,
    pub poll_start_date: u64,
    pub poll_end_date: u64,
    pub metadata: String,
    pub vote_count: u16,
    pub bump: u8,
    // pub vote_bump: u8,
}

impl Space for ChangePoll {
    const INIT_SPACE: usize = 8 + (4 + 32) + 8 + 8 + (4 + 32) + 2 + 1 + 1;
}

#[account]
pub struct ChangePollVote {
    pub voter: Pubkey,
    pub vote: String,
    pub bump: u8,
}

impl Space for ChangePollVote {
    const INIT_SPACE: usize = 8 + 32 + (4 + 32) + 1;
}
