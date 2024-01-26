use anchor_lang::prelude::*;

pub use crate::state::{ Creator, ProjectDAO, ProjectDAOUpdate };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, update_metadata: String)]
pub struct PostUpdate<'info> {
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
        space = ProjectDAOUpdate::INIT_SPACE,
        seeds = [
            b"projectdaoupdate",
            update_metadata.as_str().as_bytes(),
            project_dao.key().as_ref(),
        ],
        bump
    )]
    project_dao_update: Account<'info, ProjectDAOUpdate>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> PostUpdate<'info> {
    pub fn post_update(&mut self, update_metadata: String, bumps: &PostUpdateBumps) -> Result<()> {
        // Increment the number of updates on the project DAO
        let project_dao = &mut self.project_dao;
        project_dao.number_of_updates = project_dao.number_of_updates.checked_add(1).unwrap();

        // Set the reward backed account
        self.project_dao_update.set_inner(ProjectDAOUpdate {
            update_metadata,
            bump: bumps.project_dao_update,
        });
        Ok(())
    }
}
