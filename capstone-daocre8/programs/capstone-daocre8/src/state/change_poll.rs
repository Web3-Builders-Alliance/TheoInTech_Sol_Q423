use anchor_lang::prelude::*;

#[account]
pub struct ChangePoll {
    pub change_poll_idx: String,
    pub poll_start_date: u64,
    pub poll_end_date: u64,
    pub metadata: String,
    pub total_vote_count: u16,
    pub bump: u8,
    pub option_bump: u8,
}

impl Space for ChangePoll {
    const INIT_SPACE: usize = 8 + (4 + 32) + 8 + 8 + (4 + 32) + 2 + 1 + 1;
}

#[account]
pub struct ChangePollOption {
    pub option: String,
    pub option_vote_count: u16,
    pub bump: u8,
}

impl Space for ChangePollOption {
    const INIT_SPACE: usize = 8 + (4 + 32) + 2 + 1;
}

#[account]
pub struct ChangePollVote {
    pub voter: Pubkey,
    pub vote: u8,
    pub bump: u8,
}

impl Space for ChangePollVote {
    const INIT_SPACE: usize = 8 + 32 + 1 + 1;
}
