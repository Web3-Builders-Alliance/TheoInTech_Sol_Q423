use anchor_lang::prelude::*;

#[account]
pub struct ProjectDAO {
    pub admin: PubKey,
    pub name: String,
    pub funding_goal: u64,
    pub initial_capital: u64,
    pub funding_start_date: u64,
    pub funding_end_date: u64,
    pub detail_metadata: String,
    pub updates_metadata: String,
    pub seed: u64,
    pub bump: u8,
    pub treasury_bump: u8,
    pub milestones_bump: u8,
    pub rewards_bump: u8,
}

impl Space for ProjectDAO {
    const INIT_SPACE: usize =
        8 + 32 + (4 + 32) + 8 + 8 + 8 + 8 + (4 + 32) + (4 + 32) + 8 + 1 + 1 + 1 + 1;
}
