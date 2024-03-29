use anchor_lang::prelude::*;

#[account]
pub struct Milestone {
    pub milestone_idx: String,
    pub project: Pubkey,
    pub fund_disbursed: u64,
    pub receiver: Pubkey,
    pub deadline: u64,
    pub milestone_metadata: String,
    pub bump: u8,
    // pub milestone_poll_bump: Option<u8>,
}

impl Space for Milestone {
    const INIT_SPACE: usize = 8 + (4 + 32) + 32 + 8 + 32 + 8 + (4 + 32) + 1;
}

#[account]
pub struct MilestonePoll {
    pub milestone_poll_idx: String,
    pub poll_start_date: u64,
    pub poll_end_date: u64,
    pub milestone_polls_metadata: String,
    pub total_vote_count: u16,
    pub bump: u8,
    // pub vote_bump: u8,
}

impl Space for MilestonePoll {
    const INIT_SPACE: usize = 8 + (4 + 32) + 8 + 8 + (4 + 32) + 2 + 1 + 1;
}

#[account]
pub struct MilestonePollVote {
    pub voter: Pubkey,
    pub vote: u8,
    pub bump: u8,
}

impl Space for MilestonePollVote {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1;
}
