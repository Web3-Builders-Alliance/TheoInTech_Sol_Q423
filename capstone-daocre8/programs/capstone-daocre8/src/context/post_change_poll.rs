use anchor_lang::prelude::*;

pub use crate::state::{ Creator, ProjectDAO, ChangePoll };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, change_poll_idx: String)]
pub struct PostChangePoll<'info> {
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
        init,
        payer = signer,
        space = ChangePoll::INIT_SPACE,
        seeds = [b"changepoll", change_poll_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    change_poll: Account<'info, ChangePoll>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> PostChangePoll<'info> {
    pub fn post_change_poll(
        &mut self,
        change_poll_idx: String,
        poll_start_date: u64,
        poll_end_date: u64,
        metadata: String,
        bumps: &PostChangePollBumps
    ) -> Result<()> {
        self.change_poll.set_inner(ChangePoll {
            change_poll_idx,
            poll_start_date,
            poll_end_date,
            metadata,
            vote_count: 0,
            bump: bumps.change_poll,
        });

        Ok(())
    }
}
