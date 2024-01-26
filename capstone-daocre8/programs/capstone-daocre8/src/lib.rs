use anchor_lang::prelude::*;

declare_id!("AwjhMae1WRzopt1CvbCxnHbwys11yqdFTiJtNgTRgzJR");

pub mod state;
pub mod context;
pub mod errors;

pub use context::*;
pub use errors::*;

#[program]
pub mod capstone_daocre8 {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        // Project DAO
        project_dao_idx: String,
        funding_goal: u64,
        initial_capital: u64,
        funding_start_date: u64,
        funding_end_date: u64,
        detail_metadata: String,
        updates_metadata: String,
        // Milestones
        milestone_idx: String,
        fund_disbursed: u64,
        deadline: u64,
        milestone_metadata: String,
        // Rewards
        reward_idx: String,
        price: u64,
        reward_metadata: String,
        vote_weight: u8,
        // Deposit fee
        fee: u64
    ) -> Result<()> {
        ctx.accounts.initialize_creator(&ctx.bumps);
        ctx.accounts.create_project_dao(
            project_dao_idx,
            funding_goal,
            initial_capital,
            funding_start_date,
            funding_end_date,
            detail_metadata,
            updates_metadata,
            &ctx.bumps
        );
        ctx.accounts.create_milestone(
            milestone_idx,
            fund_disbursed,
            deadline,
            milestone_metadata,
            &ctx.bumps
        );
        ctx.accounts.create_reward(reward_idx, price, reward_metadata, vote_weight, &ctx.bumps);
        ctx.accounts.deposit_fee(fee)
    }
}
