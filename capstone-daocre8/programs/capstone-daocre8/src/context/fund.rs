use anchor_lang::{ prelude::*, system_program::{ Transfer, transfer } };
use mpl_bubblegum::{
    instructions::{ MintV1Cpi, MintV1CpiAccounts, MintV1InstructionArgs },
    types::MetadataArgs,
};

pub use crate::state::{ Creator, ProjectDAO, Reward, RewardBacked };
pub use crate::errors::{ ProjectDAOError, RewardError };

#[derive(Accounts)]
#[instruction(project_dao_idx: String, creator: Pubkey, reward_idx: String)]
pub struct Fund<'info> {
    #[account(mut)]
    backer: Signer<'info>,
    /*
     * ProjectDAO
     * qq: If I want the ProjectDAO to be token-gated using the NFT given to the creator, should I simply add it as a constraint or as part of a seed? What's the difference?
     */
    #[account(
        seeds = [b"projectdao", creator.key().as_ref(), project_dao_idx.as_str().as_bytes()],
        bump = project_dao.bump
    )]
    project_dao: Account<'info, ProjectDAO>, // Consider using Box if we reach the limit
    /*
     * Funding
     */
    #[account(seeds = [b"treasury", project_dao.key().as_ref()], bump)]
    treasury: SystemAccount<'info>,
    #[account(
        seeds = [b"reward", reward_idx.as_str().as_bytes(), project_dao.key().as_ref()],
        bump = reward.bump
    )]
    reward: Account<'info, Reward>,
    #[account(
        init,
        payer = backer,
        space = RewardBacked::INIT_SPACE,
        seeds = [
            b"reward_backed",
            backer.key().as_ref(),
            reward.key().as_ref(),
            project_dao.key().as_ref(),
        ],
        bump
    )]
    reward_backed: Account<'info, RewardBacked>,
    /*
     * System
     */
    system_program: Program<'info, System>,
}

impl<'info> Fund<'info> {
    pub fn fund_project(&mut self, bumps: &FundBumps) -> Result<()> {
        // Increment the number of backers on the reward
        let reward = &mut self.reward;
        reward.number_of_backers = reward.number_of_backers.checked_add(1).unwrap();

        // Set the reward backed account
        self.reward_backed.set_inner(RewardBacked {
            backer: self.backer.key(),
            bump: bumps.reward_backed,
        });

        // Transfer the SOL from the backer to the treasury
        let accounts = Transfer {
            from: self.backer.to_account_info(),
            to: self.treasury.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(cpi_ctx, self.reward.price)?;

        Ok(())
    }

    pub fn mint_project_nft(&mut self, bumps: &FundBumps) -> Result<()> {
        todo!("Implement this function");
        /*
         * 1. Mint the cNFT for the reward
         */

        Ok(())
    }
}
