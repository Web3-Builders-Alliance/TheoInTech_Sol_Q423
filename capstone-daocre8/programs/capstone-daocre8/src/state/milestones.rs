use anchor_lang::prelude::*;

#[account]
pub struct Milestones {
    pub project: PubKey,
    pub fund_disbursed: u64,
    pub receiver: PubKey,
    pub deadline: u64,
    pub metadata: String,
    pub bump: u8,
    pub milestone_polls_bump: u8,
}

impl Space for Milestones {
    const INIT_SPACE: usize = 8 + 32 + 8 + 32 + 8 + (4 + 32) + 1 + 1;
}

#[account]
pub struct MilestonePolls {
    pub poll_start_date: u64,
    pub poll_end_date: u64,
    pub metadata: String,
    pub vote_count: u16,
    pub bump: u8,
    pub vote_bump: u8,
}

impl Space for MilestonePolls {
    const INIT_SPACE: usize = 8 + 8 + 8 + (4 + 32) + 2 + 1 + 1;
}

#[account]
pub struct MilestonePollsVote {
    pub voter: PubKey,
    pub vote: u8,
    pub bump: u8,
}

impl Space for MilestonePollsVote {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1;
}
