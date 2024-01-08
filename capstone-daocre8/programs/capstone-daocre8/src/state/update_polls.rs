use anchor_lang::prelude::*;

#[account]
pub struct UpdatePolls {
    pub poll_start_date: u64,
    pub poll_end_date: u64,
    pub metadata: String,
    pub vote_count: u16,
    pub bump: u8,
    pub vote_bump: u8,
}

impl Space for UpdatePolls {
    const INIT_SPACE: usize = 8 + 8 + 8 + (4 + 32) + 2 + 1 + 1;
}

#[account]
pub struct UpdatePollsVote {
    pub voter: PubKey,
    pub vote: String,
    pub bump: u8,
}

impl Space for UpdatePollsVote {
    const INIT_SPACE: usize = 8 + 32 + (4 + 32) + 1;
}
