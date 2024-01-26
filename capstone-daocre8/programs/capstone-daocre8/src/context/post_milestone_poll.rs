use anchor_lang::prelude::*;

pub use crate::state::{ Creator, ProjectDAO, Milestone, MilestonePoll };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, milestone_idx: String, milestone_poll_idx: String)]
pub struct PostMilestonePoll<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    /*
     * Creator
     */
    #[account(seeds = [b"creator", signer.key().as_ref()], bump = creator.bump)]
    creator: Account<'info, Creator>,
    /*
     * ProjectDAO
     * qq: If I want the ProjectDAO to be token-gated using the NFT given to the creator, should I simply add it as a constraint or as part of a seed? What's the difference?
     */
    #[account(
        seeds = [b"projectdao", creator.key().as_ref(), project_dao_idx.as_str().as_bytes()],
        bump = project_dao.bump
    )]
    project_dao: Account<'info, ProjectDAO>, // Consider using Box if we reach the limit
    #[account(
        seeds = [b"milestone", milestone_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    milestone: Account<'info, Milestone>,
    #[account(
        init,
        payer = signer,
        space = MilestonePoll::INIT_SPACE,
        seeds = [
            b"milestonepoll",
            milestone.key().as_ref(),
            milestone_poll_idx.as_str().as_bytes(),
            project_dao.key().as_ref(),
        ],
        bump
    )]
    milestone_poll: Account<'info, MilestonePoll>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> PostMilestonePoll<'info> {
    pub fn post_milestone_poll(
        &mut self,
        milestone_idx: String,
        milestone_poll_idx: String,
        poll_start_date: u64,
        poll_end_date: u64,
        milestone_polls_metadata: String,
        bumps: &PostMilestonePollBumps
    ) -> Result<()> {
        self.milestone_poll.set_inner(MilestonePoll {
            milestone_poll_idx,
            poll_start_date,
            poll_end_date,
            milestone_polls_metadata,
            vote_count: 0,
            bump: bumps.milestone_poll,
        });

        Ok(())
    }
}
