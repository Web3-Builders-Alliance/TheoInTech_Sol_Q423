use anchor_lang::prelude::*;

pub use crate::state::{ Creator, ProjectDAO, Milestone, MilestonePoll, MilestonePollVote };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, milestone_idx: String, milestone_poll_idx: String)]
pub struct VoteMilestonePoll<'info> {
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
        seeds = [
            b"milestonepoll",
            milestone.key().as_ref(),
            milestone_poll_idx.as_str().as_bytes(),
            project_dao.key().as_ref(),
        ],
        bump
    )]
    milestone_poll: Account<'info, MilestonePoll>,
    #[account(
        init,
        payer = signer,
        space = MilestonePollVote::INIT_SPACE,
        seeds = [b"milestonepollvote", milestone_poll.key().as_ref(), project_dao.key().as_ref()],
        bump
    )]
    milestone_poll_vote: Account<'info, MilestonePollVote>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> VoteMilestonePoll<'info> {
    pub fn vote_milestone_poll(
        &mut self,
        milestone_idx: String,
        milestone_poll_idx: String,
        vote: u8,
        bumps: &VoteMilestonePollBumps
    ) -> Result<()> {
        let milestone_poll = &mut self.milestone_poll;
        milestone_poll.total_vote_count = milestone_poll.total_vote_count.checked_add(1).unwrap();

        self.milestone_poll_vote.set_inner(MilestonePollVote {
            voter: self.signer.key(),
            vote,
            bump: bumps.milestone_poll_vote,
        });
        Ok(())
    }
}
