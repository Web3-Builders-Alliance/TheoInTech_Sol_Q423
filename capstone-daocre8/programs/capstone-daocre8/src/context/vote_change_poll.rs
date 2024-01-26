use anchor_lang::prelude::*;

pub use crate::state::{ Creator, ProjectDAO, ChangePoll, ChangePollOption, ChangePollVote };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, change_poll_idx: String)]
pub struct VoteChangePoll<'info> {
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
        seeds = [b"changepoll", change_poll_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump
    )]
    change_poll: Account<'info, ChangePoll>,
    #[account(
        seeds = [b"changepolloption", change_poll.key().as_ref(), project_dao.key().as_ref()],
        bump
    )]
    change_poll_option: Account<'info, ChangePollOption>,
    #[account(
        init,
        payer = signer,
        space = ChangePollVote::INIT_SPACE,
        seeds = [b"changepollvote", change_poll.key().as_ref(), project_dao.key().as_ref()],
        bump
    )]
    change_poll_vote: Account<'info, ChangePollVote>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> VoteChangePoll<'info> {
    pub fn vote_change_poll_option(
        &mut self,
        change_idx: String,
        change_poll_idx: String,
        vote: u8,
        bumps: &VoteChangePollBumps
    ) -> Result<()> {
        let change_poll = &mut self.change_poll;
        let change_poll_option = &mut self.change_poll_option;

        change_poll.total_vote_count = change_poll.total_vote_count.checked_add(1).unwrap();
        change_poll_option.option_vote_count = change_poll_option.option_vote_count
            .checked_add(1)
            .unwrap();

        self.change_poll_vote.set_inner(ChangePollVote {
            voter: self.signer.key(),
            vote,
            bump: bumps.change_poll_vote,
        });
        Ok(())
    }
}
