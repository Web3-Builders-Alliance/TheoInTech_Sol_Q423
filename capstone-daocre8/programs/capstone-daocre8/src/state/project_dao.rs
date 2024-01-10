use anchor_lang::prelude::*;

#[account]
pub struct ProjectDAO {
    pub admin: PubKey,
    pub identifier: String,
    pub funding_goal: u64,
    pub initial_capital: u64,
    pub funding_start_date: u64,
    pub funding_end_date: u64,
    pub detail_metadata: String,
    pub updates_metadata: String,
}

impl Space for ProjectDAO {
    const INIT_SPACE: usize = 8 + 32 + (4 + 32) + 8 + 8 + 8 + 8 + (4 + 32) + (4 + 32);
}
